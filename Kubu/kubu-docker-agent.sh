#!/bin/bash
set -e

LOGFILE="./kubu-docker.log"
echo "===== $(date) =====" >> $LOGFILE

# Fetch latest SHA digest for KUBU image
LATEST_SHA=$(curl -s -H "Accept: application/vnd.oci.image.manifest.v1+json" \
  https://ghcr.io/v2/web4application/kubu/manifests/main \
  | jq -r '.config.digest')

echo "Latest SHA: $LATEST_SHA" >> $LOGFILE

# Pull image if not present locally
if ! docker image inspect ghcr.io/web4application/kubu@$LATEST_SHA > /dev/null 2>&1; then
    echo "Pulling latest KUBU image..." | tee -a $LOGFILE
    docker pull ghcr.io/web4application/kubu@$LATEST_SHA | tee -a $LOGFILE
fi

# Remove existing container
docker rm -f kubu-dev || true

# Build local container from devcontainer if needed
docker build -t kubu-local .devcontainer/ | tee -a $LOGFILE

# Run container
docker run -d --name kubu-dev -v $(pwd):/workspace -p 8000:8000 kubu-local | tee -a $LOGFILE

echo "KUBU container is up-to-date and running." | tee -a $LOGFILE
