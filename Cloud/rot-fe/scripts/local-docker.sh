#!/bin/bash

# env file path location to script location
env_file_path=$(dirname "$0")/../.env

docker build -t rot-fe .
docker run -p 80:80 --env-file $env_file_path  rot-fe 