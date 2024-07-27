#!/bin/bash

docker build --load -t mqtt2midi:latest .

docker run -ti -v $PWD:/mqtt2midi -w /mqtt2midi --env USERID=$(id -u ${USER}) --env DIR=$PWD mqtt2midi:latest