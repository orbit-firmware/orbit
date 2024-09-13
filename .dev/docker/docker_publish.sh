#!/bin/bash

docker build -t rmk .
docker login
docker tag rmk rmkfirmware/rmk:latest
docker push rmkfirmware/rmk:latest