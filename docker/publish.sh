#!/bin/bash

docker build -t orbit .
docker login
docker tag orbit orbitfirmware/orbit:latest
docker push orbitfirmware/orbit:latest