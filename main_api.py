from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

app = FastAPI()

# CORS for frontend integration
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class UploadPayload(BaseModel):
    filename: str
    content: str

@app.get("/api/assistant/start")
def launch_ai():
    return {"message": "ProjectPilot AI assistant activated!"}

@app.post("/api/data/upload")
def upload_file(data: UploadPayload):
    # Normally, you'd store data to a DB or process it
    if not data.content:
        raise HTTPException(status_code=400, detail="Empty content")
    return {"status": "success", "filename": data.filename}

@app.get("/api/health")
def health():
    return {"status": "ok"}
