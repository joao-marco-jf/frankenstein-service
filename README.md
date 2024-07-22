# Frankenstein Service

O Frankenstein Service é uma aplicação backend desenvolvida com Rust e Actix Web, que utiliza PostgreSQL como banco de dados. Este projeto fornece endpoints para autenticação e manipulação de tarefas, e pode ser executado usando Docker e Docker Compose.

## Sumário

- [Pré-requisitos](#pré-requisitos)
- [Configuração do Ambiente](#configuração-do-ambiente)
- [Instruções de Uso](#instruções-de-uso)
- [Executando Testes](#executando-testes)
- [Contribuição](#contribuição)
- [Licença](#licença)

## Pré-requisitos

Antes de iniciar, certifique-se de ter o seguinte instalado:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Configuração do Ambiente

### Arquivo `.env`

Crie um arquivo `.env` na raiz do projeto com as seguintes variáveis (ajuste conforme necessário):

```env
DATABASE_URL=postgres://frankenstein:frankenstein@localhost:5432/frankenstein
SECRET_KEY=mysecretkey
```

### Estrutura de Diretórios

O projeto possui a seguinte estrutura:

```
/frankenstein-service
|-- src/
|   |-- main.rs
|   |-- config.rs
|   |-- auth/
|   |   |-- handlers.rs
|   |   |-- mod.rs
|   |   |-- models.rs
|   |   |-- services.rs
|   |-- tasks/
|       |-- handlers.rs
|       |-- mod.rs
|       |-- models.rs
|-- db/
|   |-- init.sql
|-- .env
|-- Cargo.toml
|-- docker-compose.yml
|-- README.md
```

### Docker Compose

Para iniciar a aplicação com Docker e Docker Compose:

1. **Construir e iniciar o contêiner**:

    ```sh
    docker-compose up -d
    ```

2. **Parar o contêiner**:

    ```sh
    docker-compose down
    ```

3. **Verificar contêiner**:

    ```sh
    docker-compose ps
    ```

4. **Executar comandos no contêiner do banco de dados**:

    ```sh
    docker exec -it frankenstein-db psql -U frankenstein -d frankenstein
    ```

## Instruções de Uso

### Endpoints

1. **Login**

   - **Método**: `POST`
   - **URL**: `/login`
   - **Corpo da Requisição**:

     ```json
     {
       "username": "Admin",
       "password": "123456"
     }
     ```

   - **Resposta**:

     ```json
     {
       "status": "success",
       "message": "Login successful",
       "data": {
         "token": "your.jwt.token"
       }
     }
     ```

2. **Verificar Token**

   - **Método**: `GET`
   - **URL**: `/verify_token`
   - **Cabeçalhos**:

     ```
     Authorization: Bearer your.jwt.token
     ```

   - **Resposta**:

     ```json
     {
       "status": "success",
       "message": "Token is valid",
       "data": {
         "claims": "token_data.claims"
       }
     }
     ```

3. **Obter Tarefas**

   - **Método**: `GET`
   - **URL**: `/tasks`

   - **Resposta**:

     ```json
     {
       "status": "success",
       "message": "Tasks fetched successfully",
       "data": [
         {
           "id": 1,
           "title": "Task Title",
           "description": "Task Description",
           "completed": false
         }
       ]
     }
     ```

4. **Criar Tarefa**

   - **Método**: `POST`
   - **URL**: `/tasks`
   - **Corpo da Requisição**:

     ```json
     {
       "title": "Task Title",
       "description": "Task Description"
     }
     ```

   - **Resposta**:

     ```json
     {
       "status": "success",
       "message": "Task created successfully"
     }
     ```

## Executando Aplicação

Para executar a aplicação, use:

```sh
cargo run
```

## Contribuição

Se você deseja contribuir para o projeto, siga estes passos:

1. Faça um fork do repositório.
2. Crie uma nova branch para suas alterações (`git checkout -b minha-nova-feature`).
3. Faça suas alterações e adicione testes, se aplicável.
4. Faça um commit das suas alterações (`git commit -am 'Adiciona nova feature'`).
5. Faça um push para a branch (`git push origin minha-nova-feature`).
6. Crie um Pull Request.

## Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
