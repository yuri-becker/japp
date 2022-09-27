#!/bin/bash
#
# Copyright (C) 2022 - This file is part of "JAPP".
#
# "JAPP" is free software: you can redistribute it and/or modify it under the
#  terms of version 3 of the GNU Affero General Public License as published by the
#  Free Software Foundation.
#
# "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
#  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
#  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
#   details.
#
# You should have received a copy of the GNU Affero General Public License
# along with JAPP.  If not, see http://www.gnu.org/licenses/.
#

set -e

if test -f /assets/iconsapp.yaml.original; then
  rm /assets/iconsapp.yaml
  mv /assets/iconsapp.yaml.original /assets/iconsapp.yaml
fi

echo "$(tput setaf 7)Checking if all tools are installed..."
if ! type yarn > /dev/null 2>&1; then
  echo "$(tput setaf 1)❌  yarn is not installed. Please see https://yarnpkg.com/getting-started/install"
  exit 1
else
  echo "$(tput setaf 2)✔ yarn is installed"
fi
if ! type gcloud > /dev/null 2>&1; then
  echo -e "$(tput setaf 1)❌  Google Cloud CLI is not installed. Please see https://cloud.google.com/sdk/docs/install"
  exit 1
else
  echo "$(tput setaf 2)✔ Google Cloud CLI is installed"
fi

echo ""
echo "$(tput setaf 7)Checking if all secrets are defined..."
# Get secret names from app.yaml
envVars=$(grep --only-matching --extended-regexp -e '\$(\w+)$' /assets/iconsapp.yaml)

for i in $envVars ; do
  if gcloud secrets versions access latest --secret="$(echo "$i" | sed s/^/assets/icons/)" >/dev/null 2>&1; then
    echo "$(tput setaf 2)✔ $(echo "$i" | sed s/^/assets/icons/) is defined."
  else
      echo "$(tput setaf 1)❌  $(echo "$i" | sed s/^/assets/icons/) is not defined. Please define it using"
      echo "$(tput setaf 7)gcloud secrets create $(echo "$i" | sed s/^/assets/icons/)"
      echo "gcloud secrets versions add $(echo "$i" | sed s/^/assets/icons/) --data-file='/path/to/file-with-secret-content.txt'"
      exit 1
  fi
  done

echo ""
echo "$(tput setaf 7)Building web...$(tput sgr0)"
yarn --cwd /assets/iconsweb build | sed -e 's/^/[web] /;'

echo ""
echo "$(tput setaf 7)Preparing deployment...$(tput sgr0)"
# Copy app.yaml for modifying
cp /assets/iconsapp.yaml /assets/iconsapp.yaml.original

# Insert secrets
for i in $envVars ; do
  value=$(gcloud secrets versions access latest --secret="$(echo $i | sed s/^/assets/icons/)")
  sed -i '' s/"$i"/"$value"/ /assets/iconsapp.yaml
done


# Deploy
echo ""
echo "$(tput setaf 7)Deploying...$(tput sgr0)"
gcloud app deploy || true

# Remove file and restore original
cp /assets/iconsapp.yaml.original /assets/iconsapp.yaml
rm /assets/iconsapp.yaml.original

echo ''
echo '🎉 Finished'
