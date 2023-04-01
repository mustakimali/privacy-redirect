#!/bin/bash
set -e

get_version() {
  date '+%y%m%d_%H%M'
}

if [[ -z "$dock_version" ]]; then
    dock_version=$(get_version)
fi

echo "Building image verion:$dock_version"

docker build . -t privacy-redirect

docker tag privacy-redirect mustakimali/privacy-redirect:latest
docker push mustakimali/privacy-redirect:latest

tag="mustakimali/privacy-redirect:$dock_version"
docker tag privacy-redirect $tag
docker push mustakimali/privacy-redirect:$dock_version

docker rmi privacy-redirect
docker rmi mustakimali/privacy-redirect:latest

echo "Tagged mustakimali/privacy-redirect:$dock_version"

kubectl -n privacy-redirect set image deployments/privacy-redirect privacy-redirect=mustakimali/privacy-redirect:${dock_version}
kubectl -n privacy-redirect rollout status deployments/privacy-redirect -w

echo "Version: $dock_version"
