'use client';

import { useEffect, useState } from 'react';

type Todo = {
  id: number;
  title: string;
  done: boolean;
};

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    fetch('http://localhost:3001/todos')
      .then(res => res.json())
      .then(data => setTodos(data))
      .catch(err => console.error('Erro ao buscar tarefas:', err));
  }, []);

  return (
    <main style={{ padding: '2rem' }}>
      <h1>Lista de Tarefas</h1>
      <ul>
        {todos.map(todo => (
          <li key={todo.id}>
            {todo.title} {todo.done ? '✅' : '❌'}
          </li>
        ))}
      </ul>
    </main>
  );
}
