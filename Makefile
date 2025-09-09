.PHONY: all run clean test lint \
        python cpp ruby rust \
        docker docker-build docker-run docker-push \
        helm k8s-deploy k8s-delete deploy \
        prod-install dev-install freeze \
        docs docs-serve openapi redoc \
        universe help

# =========================================================
# Core
# =========================================================
all: python cpp ruby rust ## Build all supported targets

run: run-python run-cpp run-ruby run-rust ## Run everything

clean: ## Clean all build artifacts
	@echo "Cleaning..."
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -exec rm -r {} +
	rm -f app
	rm -rf .bundle vendor build dist site target
	docker rmi kubuverse:latest || true

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
	cargo build --release --manifest-path=contracts/Cargo.toml

run-rust:
	cargo test --manifest-path=contracts/Cargo.toml

# =========================================================
# Docker
# =========================================================
docker: docker-build ## Build Docker image

docker-build:
	docker build -t kubuverse:latest .

docker-run:
	docker run -it --rm -p 8000:8000 kubuverse:latest

docker-push:
	docker tag kubuverse:latest ghcr.io/web4application/kubuverse:latest
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

docs: openapi ## Build documentation (Sphinx or MkDocs)
	@echo "Building documentation..."
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
	@echo "Generating Redoc static HTML..."
	@mkdir -p docs
	npx redoc-cli bundle docs/openapi.json -o docs/api.html

# =========================================================
# Meta
# =========================================================
universe: clean all test lint docker docs redoc ## Clean → Build → Test → Lint → Docker → Docs

help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  make %-14s %s\n", $$1, $$2}'

