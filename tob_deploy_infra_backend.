#!/usr/bin/env bash
# ─────────────────────────────────────────────
# 🚀 Kubuverse Full Deployment Pipeline
# Production-Ready: Backend + AI + DB + Frontend + SSL + Secrets + Canary
# ─────────────────────────────────────────────

set -e

# === CONFIG ===
export PROJECT_ID="kubuverse-prod"
export REGION="us-central1"
export GKE_CLUSTER="kubuverse-cluster"
export DOMAIN="api.kubu-hai.com"
export AI_DOMAIN="ai.kubu-hai.com"
export FRONTEND_DOMAIN="kubuverse.io"
export BRANCH_NAME="main"
export REPO_URL="https://github.com/Web4application/KubuVerse.git"
export STATE_BUCKET="kubuverse-tfstate"

# === DATABASE / SECRETS ===
export DB_INSTANCE="kubuverse-db"
export DB_NAME="kubuverse"
export DB_USER="kubuuser"
export DB_PASSWORD="supersecurepassword123"

# === DEPLOYMENT SETTINGS ===
export TERRAFORM_DIR="infra/terraform"
export BACKEND_DIR="backend"
export AI_DIR="ai"
export FRONTEND_DIR="frontend/aura"

# === SETUP ENVIRONMENT ===
apt-get update -qq
apt-get install -y git docker.io kubectl unzip curl jq google-cloud-sdk python3-pip -qq
pip3 install --quiet alembic psycopg2-binary python-dotenv

# === AUTHENTICATION ===
gcloud auth login --brief || true
gcloud auth configure-docker $REGION-docker.pkg.dev
gcloud container clusters get-credentials $GKE_CLUSTER --region $REGION --project $PROJECT_ID

# === TERRAFORM INFRA ===
cd $TERRAFORM_DIR
terraform init -backend-config="bucket=${STATE_BUCKET}"
terraform apply -auto-approve -var "region=${REGION}" -var "project_id=${PROJECT_ID}"
cd ../../

# === DATABASE & K8S SECRETS ===
gcloud sql instances create $DB_INSTANCE --database-version=POSTGRES_15 --tier=db-f1-micro --region=$REGION || true
gcloud sql users create $DB_USER --instance=$DB_INSTANCE --password=$DB_PASSWORD || true
gcloud sql databases create $DB_NAME --instance=$DB_INSTANCE || true

kubectl create secret generic kubu-db-secret \
  --from-literal=DB_USER=$DB_USER \
  --from-literal=DB_PASSWORD=$DB_PASSWORD \
  --from-literal=DB_NAME=$DB_NAME \
  --from-literal=DB_HOST=$DB_INSTANCE \
  --dry-run=client -o yaml | kubectl apply -f -

# === BACKEND DEPLOY ===
cd $BACKEND_DIR
docker build -t $REGION-docker.pkg.dev/$PROJECT_ID/backend/kubuverse-backend:latest .
docker push $REGION-docker.pkg.dev/$PROJECT_ID/backend/kubuverse-backend:latest

# Apply blue-green deployment strategy
kubectl apply -f k8s/backend-deployment.yaml
kubectl rollout status deployment/kubuverse-backend

# === ALEMBIC MIGRATIONS ===
export DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_INSTANCE}:5432/${DB_NAME}"
alembic upgrade head

# === AI MODULE DEPLOY ===
cd ../$AI_DIR
docker build -t $REGION-docker.pkg.dev/$PROJECT_ID/ai/kubuverse-ai:latest .
docker push $REGION-docker.pkg.dev/$PROJECT_ID/ai/kubuverse-ai:latest
kubectl apply -f k8s/ai-deployment.yaml
kubectl rollout status deployment/kubuverse-ai

# === FRONTEND DEPLOY ===
cd ../$FRONTEND_DIR
flutter build web --release
gsutil rsync -r build/web gs://$FRONTEND_DOMAIN

# === DOMAIN & SSL ===
kubectl annotate ingress kubuverse-backend kubernetes.io/ingress.global-static-ip-name=kubuverse-ip || true
kubectl annotate ingress kubuverse-backend networking.gke.io/managed-certificates=kubuverse-cert || true
kubectl annotate ingress kubuverse-ai kubernetes.io/ingress.global-static-ip-name=kubuverse-ai-ip || true
kubectl annotate ingress kubuverse-ai networking.gke.io/managed-certificates=kubuverse-ai-cert || true

# === LOGGING & MONITORING ===
kubectl apply -f k8s/stackdriver-config.yaml || true

# === FINAL OUTPUT ===
echo "✅ Full Kubuverse Deployment Complete!"
echo "🌐 Backend: https://$DOMAIN"
echo "🤖 AI Gateway: https://$AI_DOMAIN"
echo "🎨 Frontend Aura: https://$FRONTEND_DOMAIN"
echo "🗄️ Database: postgresql://${DB_USER}:****@${DB_INSTANCE}:5432/${DB_NAME}"
