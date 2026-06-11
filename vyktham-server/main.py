from fastapi import FastAPI,HTTPException
from fastapi.middleware.cors import CORSMiddleware

from routers import format_markdown
app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


app.include_router(format_markdown.router)



