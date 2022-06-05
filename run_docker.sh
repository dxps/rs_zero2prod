#!/bin/sh

## This script runs the Docker image, that should be
## previously build using `./build_docker.sh` script.

docker run -p 8000:8000 rs_zero2prod

