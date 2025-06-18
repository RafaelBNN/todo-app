from contextlib import asynccontextmanager
from fastapi import FastAPI
from . import models, database, routes
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

@asynccontextmanager
async def lifespan(app: FastAPI):
    async with database.engine.begin() as conn:
        await conn.run_sync(models.Base.metadata.create_all)
    yield
    print("Cabou o lifespan")

app = FastAPI(lifespan=lifespan)

app.include_router(routes.router)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

