import express from "express";
import fetch from "node-fetch";
import cors from "cors";
import dotenv from "dotenv";
from openai import OpenAI
client = OpenAI()

response = client.responses.create(
  prompt={
    "id": "pmpt_68abe94a01508196a60556bd525932ca0cb4300eb98ca011",
    "version": "1"
  }
)

dotenv.config();
const app = express();
app.use(cors());
app.use(express.json());

const PORT = process.env.PORT || 3000;

// ---- ChatGPT mock or real API ----
app.post("/api/chat", async (req, res) => {
  const { message } = req.body;

  // Example: call OpenAI or Cloudflare AI here
  // For testing, we’ll just echo:
  res.json({ reply: `You said: "${message}". This is a test AI reply.` });
});

// ---- JokeAPI proxy ----
app.get("/api/joke", async (req, res) => {
  const { category="Any", type="Any", safeMode=false, lang="en" } = req.query;

  const safeParam = safeMode === "true" ? "&blacklistFlags=explicit" : "";
  const url = `https://v2.jokeapi.dev/joke/${category}?type=${type}&lang=${lang}${safeParam}`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
