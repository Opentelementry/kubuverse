# ──────────────────────────────
# Base Python Stage
# ──────────────────────────────
FROM python:3.11-slim as python-build
WORKDIR /app/python

COPY python/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY python/ .

# ──────────────────────────────
# Base C++ Stage
# ──────────────────────────────
FROM gcc:12 as cpp-build
WORKDIR /app/cpp

COPY cpp/ .
RUN make

# ──────────────────────────────
# Base Ruby Stage
# ──────────────────────────────
FROM ruby:3.2 as ruby-build
WORKDIR /app/ruby

COPY ruby/ .
RUN bundle install

# ──────────────────────────────
# Final Universal Dev Container
# ──────────────────────────────
FROM mcr.microsoft.com/devcontainers/universal:2

LABEL maintainer="Web4application Team <team@web4application.com>" \
      org.opencontainers.image.source="https://github.com/Web4application/kubuverse.git"

# Update & install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake g++ \
    python3.11 python3-pip python3-venv python3.11-venv \
    ruby-full \
    nodejs npm \
    git curl docker.io \
    postgresql-client redis-tools \
    && rm -rf /var/lib/apt/lists/*

# Install Bundler for Ruby
RUN gem install bundler

# Upgrade npm & add yarn
RUN npm install -g npm@latest yarn

# Set Python 3.11 as default
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 1

# ──────────────────────────────
# Copy builds from previous stages
# ──────────────────────────────
WORKDIR /app
COPY --from=python-build /app/python /app/python
COPY --from=cpp-build /app/cpp/bin /app/cpp/bin
COPY --from=ruby-build /app/ruby /app/ruby

# Copy root Makefile into container
COPY Makefile /app/Makefile

# Default workspace for development
WORKDIR /workspace

# ──────────────────────────────
# Default entrypoint → Make help
# ──────────────────────────────
CMD ["make", "-C", "/app", "help"]
