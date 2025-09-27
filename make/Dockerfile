.PHONY: all run clean test lint \
        python cpp ruby rust \
        docker docker-build docker-run docker-all docker-windows docker-push \
        helm k8s-deploy k8s-delete deploy \
        prod-install dev-install freeze \
        docs docs-serve openapi redoc \
        universe help

# =========================================================
# Compiler & Directories
# =========================================================
CC = gcc
CXX = g++
CFLAGS = -Wall -O2
CXXFLAGS = -Wall -O2 -std=c++17

SRC_DIR = src
BIN_DIR = bin
PYTHON_DIR = python
RUBY_DIR = ruby
DOCKER_IMAGE = kubuverse:latest

# =========================================================
# Core
# =========================================================
all: python cpp ruby rust ## Build all supported targets

run: run-python run-cpp run-ruby run-rust ## Run everything

clean: ## Clean all build artifacts
	@echo "🧹 Cleaning..."
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -exec rm -r {} +
	rm -f app
	rm -rf $(BIN_DIR) .bundle vendor build dist site target
	docker rmi $(DOCKER_IMAGE) || true

# =========================================================
# Python
# =========================================================
python: install-python ## Install Python deps

install-python:
	pip install -r requirements.txt || true

prod-install:
	pip install -r requirements.txt

dev-install:
	pip install -r dev-requirements.txt

freeze:
	pip freeze > requirements.lock.txt

run-python:
	python main.py

# =========================================================
# C++ (Core Engine)
# =========================================================
cpp: build-cpp ## Build C++ app

build-cpp:
	@echo "🔨 Building C++ sources..."
	mkdir -p $(BIN_DIR)
	$(CXX) $(CXXFLAGS) $(SRC_DIR)/*.cpp -o $(BIN_DIR)/kubuverse

run-cpp: build-cpp
	@echo "🚀 Running C++ binary..."
	./$(BIN_DIR)/kubuverse

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
	cargo build --release --manifest-path=contracts/Cargo.toml

run-rust:
	cargo test --manifest-path=contracts/Cargo.toml

# =========================================================
# Docker
# =========================================================
docker: docker-build ## Build Docker image

docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t $(DOCKER_IMAGE) .

docker-run:
	@echo "🐳 Running Dockerized KubuVerse..."
	docker run -it --rm -p 8000:8000 $(DOCKER_IMAGE)

docker-all:
	@echo "🐳 Building all artifacts inside Docker..."
	docker build --target final -t $(DOCKER_IMAGE) .
	docker create --name extract $(DOCKER_IMAGE)
	mkdir -p dist
	docker cp extract:/app dist/
	docker rm extract
	@echo "✅ Artifacts extracted to ./dist"

docker-windows:
	@echo "🐳 Building Windows binary inside Docker..."
	docker build --target windows-build -t $(DOCKER_IMAGE)-windows .
	docker create --name extract-win $(DOCKER_IMAGE)-windows
	mkdir -p dist/windows
	docker cp extract-win:/build/windows/runner.exe dist/windows/
	docker rm extract-win
	@echo "✅ Windows binary available at dist/windows/runner.exe"

docker-push: docker
	@echo "📦 Pushing Docker image to registry..."
	docker tag $(DOCKER_IMAGE) ghcr.io/web4application/kubuverse:latest
	docker push ghcr.io/web4application/kubuverse:latest

# =========================================================
# Helm / Kubernetes
# =========================================================
helm: helm-deploy ## Deploy via Helm

helm-deploy k8s-deploy:
	helm upgrade --install kubuverse charts/ --namespace kubuverse --create-namespace

k8s-delete:
	helm uninstall kubuverse

deploy: docker-build docker-push helm-deploy ## Full deploy pipeline

# =========================================================
# Testing & Linting
# =========================================================
test: ## Run tests (Python + Ruby + Rust)
	pytest tests/test_main.py || true
	rspec tests/test_main.rb || true
	cargo test --manifest-path=contracts/Cargo.toml

lint: ## Run linters
	flake8 main.py || true
	black --check . || true
	isort --check-only . || true
	rubocop main.rb || true
	cargo clippy --manifest-path=contracts/Cargo.toml || true

# =========================================================
# Documentation & API Schema
# =========================================================
openapi: ## Export FastAPI OpenAPI schema
	@echo "Exporting FastAPI OpenAPI schema..."
	uvicorn backend.main:app --host 127.0.0.1 --port 9000 --reload & \
	PID=$$!; \
	sleep 3; \
	curl -s http://127.0.0.1:9000/openapi.json > docs/openapi.json; \
	kill $$PID

docs: openapi ## Build documentation
	@echo "📚 Building documentation..."
	@if [ -f docs/Makefile ] || [ -f docs/make.bat ]; then \
		$(MAKE) -C docs html; \
	else \
		mkdocs build; \
	fi

docs-serve: ## Serve documentation locally
	@if [ -f docs/Makefile ] || [ -f docs/make.bat ]; then \
		$(MAKE) -C docs livehtml; \
	else \
		mkdocs serve; \
	fi

redoc: openapi ## Generate Redoc static HTML
	@echo "📘 Generating Redoc API docs..."
	@mkdir -p docs
	npx redoc-cli bundle docs/openapi.json -o docs/api.html

# =========================================================
# Meta
# =========================================================
universe: clean all test lint docker docs redoc ## Full pipeline

help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  make %-14s %s\n", $$1, $$2}'
