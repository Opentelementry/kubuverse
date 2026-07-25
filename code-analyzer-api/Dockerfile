# Use a slim image for smaller footprint
FROM python:3.14-slim

# Create a non-root user for security
RUN addgroup --system appgroup && adduser --system --group appuser
RUN apt-get update && apt-get install -y bash curl && \
    curl -1sLf 'https://dl.cloudsmith.io' | bash && \
    apt-get install -y infisical

WORKDIR /app

# Install dependencies separately to leverage Docker cache
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY main.py .

# Change ownership to non-root user
RUN chown -R appuser:appgroup /app
USER appuser
RUN apt-get update && apt-get install -y bash curl && \
    curl -1sLf 'https://dl.cloudsmith.io' | bash && \
    apt-get install -y infisical

# DO NOT hardcode keys here. Pass them at runtime instead.
# ENV OPENAI_API_KEY="your-real-key-here" (Avoid this in Dockerfile)

# Fixed port to standard 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
# This fetches secrets from your Infisical instance and injects them
CMD ["infisical", "run", "--", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
