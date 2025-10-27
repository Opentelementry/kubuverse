# Base image
FROM ubuntu:24.04

# Install core dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    sudo \
    curl \
    wget \
    git \
    unzip \
    zip \
    build-essential \
    python3 python3-pip python3-venv \
    openjdk-20-jdk \
    nodejs npm \
    golang-go \
    docker.io \
    gradle \
    maven \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Optional: configure Go environment globally
ENV GOPATH=/home/dev/kubuverse/.gogradle/project_gopath
ENV PATH="$PATH:$GOPATH/bin"

# Optional: add kubu CLI placeholder
RUN mkdir -p /usr/local/kubu && echo "echo 'KUBU CLI placeholder'" > /usr/local/kubu/kubu && chmod +x /usr/local/kubu/kubu

# Set a default shell
CMD ["/bin/bash"]
