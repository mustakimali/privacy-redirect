#!/bin/bash
set -eu pipefail

rm -rf ../out || true
mkdir ../out && mkdir ../out/v2 && mkdir ../out/v3
cp -r * ../out/v2 || true
cp -r * ../out/v3 || true
rm ../out/v2/manifest_v3.json
rm ../out/v3/manifest.json && mv ../out/v3/manifest_v3.json ../out/v3/manifest.json
rm ../out/v2/build.sh
rm -rf ../out/v2/manifest_v3
rm ../out/v3/build.sh
rm ../out/v3/script.js
