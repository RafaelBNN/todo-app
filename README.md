# Descrição

Este projeto faz parte do processo seletivo para o Rei do Pitaco. Trata-se de uma aplicação simples de lista de tarefas para organização pessoal.

# Como Rodar

[Vídeo demonstração](https://youtu.be/5J1gdAmI080)

## Configuração do ambiente

### Pré-requisitos

- Rust
- PostGreSQL
- Node.js
- npm ou yarn

### Clone o repositório

Clone o repositório e navegue até ele usando:

```bash
git clone git@github.com:RafaelBNN/todo-app.git
cd todo-app
```

### Crie o banco de dados

Acesse o Postgres e crie um banco de dados chamado `todo_db`:

```sql
CREATE DATABASE todo_db;
\c todo_db
```

Crie uma tabela chamada `todos` da seguinte forma:

```sql
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT NOW()
);
```

Ou você pode usar o script disponibilizado no repositório:

```bash
psql -U usuario -d todo_db -f database/init.sql
```

sendo `usuario` o usuário do banco.

### Configure a variável de ambiente `DATABASE_URL`

Crie um arquivo `.env` em `backend/`:

```bash
cd backend
touch .env
```

e adicione a seguinte linha (com seu usuário e senha do banco do Postgres):

```bash
DATABASE_URL=postgres://usuario:senha@localhost/todo_db
```

Você pode fazer isso usando:

```bash
echo "DATABASE_URL=postgres://usuario:senha@localhost/todo_db" > .env
```

## Rodando

Agora está tudo pronto!

Em um terminal, navegue até a pasta `backend/` e execute:

```bash
cargo build
cargo run
```

E, em outro, navegue até a pasta `frontend/` e execute o comando:

```bash
npm run dev
```

Agora, no seu navegador, acesse a URL http://localhost:3000. Você deve ver a lista de tarefas vazia. Você pode adicionar, remover ou marcar uma tarefa como concluída.

# Arquitetura

As tecnologias utilizadas foram as seguintes:

- PostGreSQL para o banco de dados, para guardar as tarefas a serem listadas;
- Rust com Axium para criar as rotas a serem chamadas no frontend e fazer a conexão com o banco;
- e React e Next.js para o frontend web

A arquitetura do sistema ficou mais ou menos assim, num diagrama de blocos:

![alt text](<img.png>)

# Versionamento

Segui uma abordagem "incremental" na construção do sistema, de modo que criei o banco de dados, o backend e o frontend independentemente e os testei manualmente antes de subir (commitar) as alterações.

Quando uma alteração precisava de mais de um commit, criei uma branch específica, e, ao final da implementação, dei merge na main.

# Uso de Inteligência Artificial

Utilizei o ChatGPT inicialmente para obter sugestões de como utilizar a stack (ou pelo menos uma parte dela) usada no Rei do Pitaco para desenvolver o projeto e para criar uma estrutura de arquivos inicial. ([sugestões iniciais do ChatGPT](https://chatgpt.com/share/6825ec82-824c-8012-b2aa-9ba6799084ed))

Também usei a função de 'Explain' do Copilot do VSCode para entender melhor o que cada parte do código fazia.

Usei o ChatGPT também pra eventuais erros de instalação, compilação, etc.

# Trabalhos futuros

Com mais tempo, eu buscaria outros projetos em cada uma dessas linguagens para saber como eles são estruturados (arquivos, diretórios, arquitetura no geral).

Também acredito que poderia entender mais o porquê de cada função ou variável. Ou estudar mais Rust, que foi uma linguagem nova para mim.

Refatorar o código do frontend também seria uma tarefa importante com o
objetivo de deixar mais clara a função de cada parte do código.