/*
 * Copyright (C) 2022 - This file is part of "JAPP".
 *
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with JAPP.  If not, see http://www.gnu.org/licenses/.
 */
use rand::seq::SliceRandom;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn animals() -> Vec<String> {
    let file = File::open("resources/animals.txt").expect("Could not read animals.txt");
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn colors() -> Vec<String> {
    let file = File::open("resources/colors.txt").unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

pub fn generate_session_name() -> String {
    format!(
        "{} {}",
        colors().choose(&mut rand::thread_rng()).unwrap(),
        animals().choose(&mut rand::thread_rng()).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use crate::usecase::generate_session_name::generate_session_name;

    #[test]
    fn should_output_random_name() {
        let name = generate_session_name();
        assert!(!name.trim().is_empty());
    }
}
