FROM python:3.11-slim

# Install Infisical CLI
RUN apt-get update && apt-get install -y curl && \
    curl -1sLf 'https://dl.cloudsmith.io' | bash && \
    apt-get install -y infisical

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py .

# The CLI fetches secrets from your self-hosted instance and provides them to Uvicorn
CMD ["infisical", "run", "--", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
