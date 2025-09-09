.PHONY: all clean run test lint \
        python cpp ruby rust docker helm deploy \
        install-python run-python build-cpp run-cpp install-ruby run-ruby build-rust run-rust \
        docker-build docker-run docker-push \
        helm-deploy universe help

# =========================================================
# Core
# =========================================================
all: python cpp ruby rust docker ## Build all supported targets

run: run-python run-cpp run-ruby run-rust docker-run ## Run everything

clean: ## Clean all build artifacts
	@echo "Cleaning..."
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -exec rm -r {} +
	rm -f app
	rm -rf .bundle vendor
	rm -rf target
	docker rmi kubuverse:latest || true

# =========================================================
# Python
# =========================================================
python: install-python ## Install Python deps

install-python:
	pip install -r requirements.txt || true

run-python:
	python main.py

# =========================================================
# C++
# =========================================================
cpp: build-cpp ## Build C++ app

build-cpp:
	g++ -Wall -std=c++17 -o app main.cpp

run-cpp: build-cpp
	./app

# =========================================================
# Ruby
# =========================================================
ruby: install-ruby ## Install Ruby deps

install-ruby:
	bundle install || true

run-ruby:
	ruby main.rb

# =========================================================
# Rust (Smart Contracts)
# =========================================================
rust: build-rust ## Build Rust contracts

build-rust:
	cd contracts && cargo build --release

run-rust: build-rust
	cd contracts && cargo test

# =========================================================
# Docker
# =========================================================
docker: docker-build ## Build Docker image

docker-build:
	docker build -t kubuverse:latest .

docker-run:
	docker run --rm -p 8000:8000 kubuverse:latest

docker-push:
	docker tag kubuverse:latest ghcr.io/web4application/kubuverse:latest
	docker push ghcr.io/web4application/kubuverse:latest

# =========================================================
# Helm / Kubernetes
# =========================================================
helm: helm-deploy ## Deploy via Helm

helm-deploy:
	helm upgrade --install kubuverse charts/ --namespace kubuverse --create-namespace

deploy: docker-build docker-push helm-deploy ## Full deploy pipeline

# =========================================================
# Testing & Linting
# =========================================================
test: ## Run tests (Python + Ruby + Rust)
	pytest tests/test_main.py
	rspec tests/test_main.rb || true
	cd contracts && cargo test

lint: ## Run linters
	flake8 main.py || true
	rubocop main.rb || true
	cd contracts && cargo clippy || true

# =========================================================
# Meta
# =========================================================
universe: all test lint deploy ## Build → Test → Lint → Deploy

help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  make %-14s %s\n", $$1, $$2}'
