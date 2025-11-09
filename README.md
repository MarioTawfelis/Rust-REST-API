# FireFleeb API

Rust + Warp + Postgres (via Diesel) API for carts, products, and users.

## Prerequisites

- Rust 1.82+ (see `rust-toolchain.toml` if you pin a version later)
- PostgreSQL 15+ or Docker if you prefer containers
- `libpq`/`pkg-config` installed locally (Diesel requires the Postgres client libs)

Copy the sample environment file (used by both `cargo run` and `docker-compose`):

```bash
cp .env.example .env
```

## Running locally

```bash
# Ensure a Postgres instance is running that matches DATABASE_URL
cargo run -p firefleeb_api
```

The server automatically runs pending Diesel migrations on start and listens on `PORT` (default `8080`).

### Running tests

```bash
cargo test
```

Integration tests under `firefleeb_api/tests/` use `testcontainers` to boot throwaway Postgres instances, so Docker must be available on the host.

## Docker

Build your container image:

```bash
docker build -t firefleeb-api .
docker run --rm -p 8080:8080 \
  -e DATABASE_URL=postgres://postgres:password@host.docker.internal:5432/firefleeb \
  firefleeb-api
```

### Docker Compose (API + Postgres)

```bash
docker compose up --build
```

The compose file loads `.env` automatically, starts Postgres + the API, and exposes the service on `http://localhost:8080`.

## CI/CD

GitHub Actions workflow (`.github/workflows/ci.yml`) runs on every push/PR:

1. `cargo fmt --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`

## `curl` commands for testing
Make sure the server is running (locally or in a container)
1. Create a user
```
curl -X POST http://localhost:8080/users \
  -H 'Content-Type: application/json' \
  -d '{"email":"demo@example.com","password":"SecretPass8"}'
```
2. Log in
```
curl -X POST http://localhost:8080/users/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"demo@example.com","password":"SecretPass8"}'
```
3. Create a cart
```
curl -X POST http://localhost:8080/carts \
  -H 'Content-Type: application/json' \
  -d '{"user_id":"<uuid returned from user creation>"}'
```
4. Add a cart item (make sure a product exists first)
```
curl -X POST http://localhost:8080/carts/<cart_id>/items \
  -H 'Content-Type: application/json' \
  -d '{"item_id":"<product_uuid>","quantity":2,"unit_price":"19.99"}'
  ```
5. Check cart totals 
```
curl http://localhost:8080/carts/<user_id>
```

6. Password reset
```
curl -X PUT http://localhost:8080/users/password-reset/<user_id> \
  -H 'Content-Type: application/json' \
  -d '{"old_password":"SecretPass8","new_password":"NewSecret9"}'
```