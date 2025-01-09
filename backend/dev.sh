#!/bin/bash

# 日志级别
export RUST_LOG=info

if [[ "$1" == "build" ]]; then
    docker build -t instant-monitor:v0.1.0 -t instant-monitor:latest .
    exit 0
fi

if [[ "$1" == "stop" ]]; then
    docker-compose down
    exit 0
fi

docker-compose up -d
docker-compose ps 