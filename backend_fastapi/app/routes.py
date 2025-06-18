from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy.future import select
from sqlalchemy import update, delete

from . import models, schemas
from .database import get_db

router = APIRouter()

@router.get("/todos", response_model=list[schemas.TodoOut])
async def list_todos(db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(models.Todo).order_by(models.Todo.id))
    return result.scalars().all()

@router.post("/todos", response_model=schemas.TodoOut, status_code=201)
async def create_todo(todo: schemas.TodoCreate, db: AsyncSession = Depends(get_db)):
    new_todo = models.Todo(title=todo.title)
    db.add(new_todo)
    await db.commit()
    await db.refresh(new_todo)
    return new_todo

@router.put("/todos/{todo_id}", response_model=schemas.TodoOut)
async def update_todo(todo_id: int, payload: schemas.TodoUpdate, db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(models.Todo).where(models.Todo.id == todo_id))
    todo = result.scalar_one_or_none()
    if not todo:
        raise HTTPException(status_code=404, detail="Tarefa não encontrada")

    todo.done = payload.done
    await db.commit()
    await db.refresh(todo)
    return todo

@router.delete("/todos/{todo_id}", status_code=204)
async def delete_todo(todo_id: int, db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(models.Todo).where(models.Todo.id == todo_id))
    todo = result.scalar_one_or_none()
    if not todo:
        raise HTTPException(status_code=404, detail="Tarefa não encontrada")

    await db.delete(todo)
    await db.commit()
