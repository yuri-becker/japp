/*
 * Copyright (C) 2023 - This file is part of "JAPP".
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 * terms of version 3 of the GNU Affero General Public License as published by the
 * Free Software Foundation.
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 * details.
 * You should have received a copy of the GNU Affero General Public License
 * long with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

use data_encoding::{BASE64URL_NOPAD, HEXLOWER};
use rand::rngs::OsRng;
use rand::Rng;
use ring::digest::SHA512_OUTPUT_LEN;
use ring::pbkdf2;
use ring::pbkdf2::Algorithm;
use std::num::NonZeroU32;

static ALGORITHM: Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
static ITERATIONS: Option<NonZeroU32> = NonZeroU32::new(100_000);

pub struct Secret {
    pub clear_text: String,
    pub db_safe_encrypted: String,
}

fn generate_cleartext() -> String {
    let mut cleartext = [0u8; 32];
    OsRng::default().fill(&mut cleartext);
    BASE64URL_NOPAD.encode(&cleartext)
}

fn encrypt_secret(cleartext: &str) -> String {
    let mut salt = [0u8; SHA512_OUTPUT_LEN];
    OsRng::default().fill(&mut salt);
    let mut pbkdf2_hash = [0u8; SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        ALGORITHM,
        ITERATIONS.unwrap(),
        &salt,
        cleartext.as_bytes(),
        &mut pbkdf2_hash,
    );
    let whole: [u8; SHA512_OUTPUT_LEN * 2] = {
        let mut whole: [u8; SHA512_OUTPUT_LEN * 2] = [0; SHA512_OUTPUT_LEN * 2];
        let (one, two) = whole.split_at_mut(pbkdf2_hash.len());
        one.copy_from_slice(&pbkdf2_hash);
        two.copy_from_slice(&salt);
        whole
    };
    HEXLOWER.encode(&whole)
}

pub fn generate_secret() -> Secret {
    let cleartext = generate_cleartext();
    Secret {
        clear_text: cleartext.to_string(),
        db_safe_encrypted: encrypt_secret(&cleartext),
    }
}

pub fn verify_secret(secret: &Secret) -> bool {
    HEXLOWER
        .decode(secret.db_safe_encrypted.as_bytes())
        .map(|decoded| {
            let (hash, salt) = decoded.as_slice().split_at(SHA512_OUTPUT_LEN);
            pbkdf2::verify(
                ALGORITHM,
                ITERATIONS.unwrap(),
                salt,
                secret.clear_text.as_bytes(),
                hash,
            )
            .map(|_| true)
            .unwrap_or(false)
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use crate::usecase::secrets::{generate_secret, verify_secret, Secret};

    #[test]
    fn should_verify_generated_secrets() {
        let generated = generate_secret();
        assert!(verify_secret(&generated));
    }

    #[test]
    fn should_not_verify_wrong_secrets() {
        assert!(!verify_secret(&Secret {
            clear_text: "Bla".to_string(),
            db_safe_encrypted: "4d07db604305534d7f2446aac438281d9ade4abb7f5e323a340e5436f27b66b6bbbbb49d65b246d2e9a147b46227cdfcbdbbe043ef509990ab498240ca03024444da1d84ee433df583b17906131b632d3c745f3f3d9dd408c4dc7cf265522231055c620e8afe2b211c695bf644a282871c7f90b8f42fc17f5add897597cebbb5".to_string()
        }))
    }

    #[test]
    fn should_verify_existing_secrets() {
        assert!(verify_secret(&Secret {
            clear_text: "VAegRhk5Ej3pfKE-XhOyj5KGY6_TjQgvLcF9sOqLkfQ".to_string(),
            db_safe_encrypted: "13cba619eda37692cc03b390d2870c247d0b0ad581439cfd33ee83ec522db4302c0aed42f61aa8e4fd55346fa10913503350a51a80a4bb8f85aedd95e09af7dad0b8b9365840e5dc03c80381fea7ae1df9241d8a0ae890657808cc271ea9d1f240c904b8d8add105c69337120404357486dfeb81d209758428771634de80c2a4".to_string()
        }))
    }
}
