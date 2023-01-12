#!/bin/bash
set -eu pipefail

mkdir - p static && cd frontend && npm -i g yarn && yarn install && yarn build && cd ..
echo
echo "First time build complete, please run 'cargo run' to start the server"
echo "The application will be availabe in 'http://localhost:8080'"
