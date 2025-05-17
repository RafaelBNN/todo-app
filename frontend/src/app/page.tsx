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
    <main style={{
      display: 'flex',
      justifyContent: 'center',
      padding: '2rem',
      fontFamily: 'sans-serif',
    }}>
      <div style={{ maxWidth: '600px', width: '100%' }}>
        <h1 style={{ fontSize: '2rem', marginBottom: '1.5rem', textAlign: 'center' }}>
          Lista de Tarefas
        </h1>
        <ul style={{ listStyle: 'none', padding: 0 }}>
          {todos.map(todo => (
            <li
              key={todo.id}
              style={{
                backgroundColor: '#32a866',
                padding: '1rem',
                marginBottom: '1rem',
                borderRadius: '8px',
                boxShadow: '0 10px 10px rgb(48, 66, 50)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
              }}
            >
              <span style={{
                textDecoration: todo.done ? 'line-through' : 'none',
                color: todo.done ? '#dbdbdb' : '#f9f9f9',
              }}>
                {todo.title}
              </span>
              <span style={{ fontSize: '1.2rem' }}>
                {todo.done ? '✅' : '🕓'}
              </span>
            </li>
          ))}
        </ul>
      </div>
      {/* <footer>
        <a
          href="https://www.flaticon.com/authors/graphics-plazza"
          target="_blank"
          rel="noopener noreferrer"
        >
          Tab icon by Graphics Plazza →
        </a>
      </footer> */}
    </main>
  );
}
