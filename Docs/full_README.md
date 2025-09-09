![Kubuverse Architecture](kubuverse-arch.png)

# 🌌 KubuVerse  

[![Build](https://img.shields.io/github/actions/workflow/status/Web4application/kubuverse/ci.yml?branch=main)](https://github.com/Web4application/kubuverse/actions)  
[![License](https://img.shields.io/github/license/Web4application/kubuverse)](LICENSE)  
![Issues](https://img.shields.io/github/issues/Web4application/kubuverse)  
![Stars](https://img.shields.io/github/stars/Web4application/kubuverse)  
![Docker](https://img.shields.io/badge/Docker-ready-blue)  
![Kubernetes](https://img.shields.io/badge/Kubernetes-supported-blueviolet)  

---

## 🖼️ Visual Overview  

<p align="center">
  <img src="docs/kubuverse-architecture.png" alt="KubuVerse Architecture" width="600"/>
</p>


KubuVerse is an open-source, AI-native, decentralized development ecosystem built on Web4/Web5 ideals. It merges intelligent tooling, multilingual support, blockchain integration, and cloud-native infrastructure to power scalable, secure, and collaborative applications.

---

## ⚡ Features

- **🧠 AI Integration**
  - Smart code suggestions and workflow automation
  - Intelligent CI/CD and container orchestration

- **🛠️ Multilingual Modules**
  - Python (FastAPI Backend)
  - Rust (Smart Contracts)
  - Dart or JS (Frontend)
  - Shell/JSON/YAML for DevOps

- **🔗 Blockchain Ready**
  - Smart contract engine
  - Wallet and token modules
  - Rust-based WASM contracts

- **📦 Kubernetes-Native**
  - Helm charts for deployment
  - Autoscaling and traffic management
  - RBAC and secrets handling

- **🧪 Developer Experience**
  - Docker Compose-powered local dev
  - GitHub Actions CI/CD with Cosign container signing
  - Internationalization via FastAPI Accept-Language detection

---

## 🧱 Tech Stack

| Layer          | Technology              |
|----------------|-------------------------|
| Backend        | Python, FastAPI, Pydantic |
| Smart Contracts| Rust/WASM               |
| Frontend       | Dart / JavaScript       |
| Database       | PostgreSQL + Alembic    |
| Caching        | Redis (optional)        |
| DevOps         | Docker, Buildx, Cosign  |
| CI/CD          | GitHub Actions          |
| Deployment     | Kubernetes + Helm       |

---

## 🧭 Architecture Overview

```text
          ┌───────────────────────┐
          │     👨‍💻  User Interface    │
          │  (Dart / JS Frontend) │
          └─────────┬─────────────┘
                    │
        [HTTP/API Requests via FastAPI]
                    │
          ┌─────────▼────────────┐
          │  🧠 Backend Service   │
          │  (Python + FastAPI)  │
          └────┬────────┬────────┘
               │        │
        ┌──────▼───┐  ┌─▼────────────┐
        │Database  │  │ Smart Contract│
        │PostgreSQL│  │ Engine (Rust) │
        └──────────┘  └──────────────┘
               │
               │
       ┌───────▼─────────┐
       │🕸️ Blockchain Layer│
       │Ethereum / WASM  │
       └─────────────────┘

          ┌────────────────────────┐
          │     🔁 CI/CD System     │
          │ GitHub Actions + Docker│
          └────────┬───────────────┘
                   │
          ┌────────▼─────────────┐
          │ 🚀 Deployment Layer   │
          │ Kubernetes + Helm    │
          └──────────────────────┘
```

---

## 🚀 Getting Started

### 🔧 Local Development

```bash
git clone https://github.com/Web4application/kubuverse.git
cd kubuverse
docker-compose up --build
```

> Requires: Docker, Python 3.10+, Git

### 🔐 Secure Signing (Optional)

```bash
cosign sign ghcr.io/<your-org>/kubuverse:<tag>
```

---

## 📘 API Documentation (FastAPI)

Access auto-generated documentation after launching:

- Swagger UI: `http://localhost:8000/docs`
- Redoc UI: `http://localhost:8000/redoc`

### Key Endpoints

| Method | Endpoint           | Description                         |
|--------|--------------------|-------------------------------------|
| `GET`  | `/users/{id}`      | Retrieve user info                  |
| `POST` | `/users/`          | Create a user                       |
| `POST` | `/auth/login`      | Authenticate user                   |
| `GET`  | `/health`          | Health check                        |
| `POST` | `/contracts/invoke`| Run smart contract logic            |

---

## 📂 Project Structure

```
kubuverse/
│
├── backend/         # FastAPI microservice
│   ├── main.py
│   ├── routes/
│   ├── models/
│   ├── services/
│   └── i18n/
│
├── contracts/       # Smart contracts in Rust
├── frontend/        # Dart / JS frontend
├── charts/          # Helm deployment charts
├── .github/         # CI/CD workflows
└── docker-compose.yml
```

---

## 🧪 Testing

- Run local unit tests with `pytest`
- Use Docker for isolated integration testing
- Check GitHub Actions for continuous testing pipeline

---

## 🚢 Deployment Guide

- **Local**: via Docker Compose
- **Cloud**: deploy Helm charts to Kubernetes clusters on GCP/AWS/DigitalOcean
- **Security**: enable Cosign signing + RBAC + secrets via Kubernetes

---

## 🤖 Contributor Handbook

### Git Workflow

```bash
# Fork → Clone → Branch
git checkout -b feature/my-feature
git commit -m "Add: my feature"
git push origin feature/my-feature
```

### Rules of Thumb
- Write tests for new features
- Follow PEP8 and Rustfmt conventions
- Submit clear PR descriptions
- Use conventional commits (e.g. `feat: add login flow`)

---

## 📜 License

This project is licensed under the [MIT License](LICENSE)

---

## 🌐 Community

- 📧 Email: [web4application@gmail.com](mailto:web4application@gmail.com)
- 🗣️ Discord: *Coming soon*

![Kubuverse Architecture](kubuverse-arch.png)
