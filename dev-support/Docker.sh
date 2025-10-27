#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

DOCKER_COMPOSE_FILE=dev-support/docker/docker-compose.yml

# Default: start interactive dev container
COMMAND=${1:-"up -d"}

docker compose -f $DOCKER_COMPOSE_FILE $COMMAND
