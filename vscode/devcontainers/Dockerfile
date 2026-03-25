ARG VARIANT=3-bullseye
FROM mcr.microsoft.com/vscode/devcontainers/python:0-${VARIANT}

ENV PYTHONUNBUFFERED=1

# Install Docker CLI for pulling and running containers
RUN apt-get update && apt-get install -y docker.io jq curl \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt /tmp/pip-tmp/requirements.txt
RUN pip3 install --no-cache-dir -r /tmp/pip-tmp/requirements.txt \
    && rm -rf /tmp/pip-tmp

# Copy the entrypoint script
COPY pull_kubu.sh /usr/local/bin/pull_kubu.sh
RUN chmod +x /usr/local/bin/pull_kubu.sh
