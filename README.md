# Tickify API 🎟

Tickify is a support ticket management API built in **Rust** using **Axum**. It allows to efficiently manage `support tickets`, `users`, and `authentication`.

## Features
- **CRUD** Operations for `Tickets` and `Users`
- **JWT Authentication** for secure user login
- **Role-based Access Control** (RBAC) for managing permissions
- **Logging** to `console` and `files` for monitoring and debugging
- **PostgreSQL** Database with `sqlx`
- **Error Handling** using `thiserror` for structured and clear error messages
- **MVC Architecture** for clean separation of concerns
- **OpenAPI (Swagger)** Documentation auto-generated with `utoipa`
- **Environment Variable** Configurations for easy environment management
- **Data Validation** using `validator` for safe input
- **Secure Password Encryption** with `argon2`
- **CORS Support** to manage cross-origin requests
- **Migrations** with `sqlx` for database versioning
- **Utility scripts** with `just`
- **Ticket export** to `PDF` and `CSV`

---

# Getting Started 🎯
## Prerequisites:

- **Rust** *(latest stable version)*
- **Docker** and **Docker Compose**
- **SQLX-cli** for migrations
- **Cargo-watch** for auto-run
- **Just** for scripts

## 1. Installation

``` bash
git clone https://github.com/allansomensi/tickify.git
cd tickify
```

For the scripts:
``` elixir
cargo install just
```

For the migrations:
``` elixir
cargo install sqlx-cli
```

For the auto-run:
``` elixir
cargo install cargo-watch
```

## 2. Build and run the Docker container:

``` elixir
just services-up
```

## 3. Start server 🚀 🚀 

``` elixir
just serve
```

---

# Running Tests 👨‍🔬

For once:
``` elixir
just test
```

For watching mode:
``` elixir
just test-watch
```

---

## API Documentation 📚

API endpoints and usage details are documented using `Swagger UI` and `OpenAPI` with `Utoipa`.

The full documentation is available in the `openapi.json` file, which can be accessed and imported as needed. Run the application and navigate to `/swagger-ui` to view the interactive Swagger documentation.
