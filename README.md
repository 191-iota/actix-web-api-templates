# actix-web-api-templates

Minimal Actix Web API templates extracted from the structure and coding style used in `lightning-git-backend`.

## Included templates

### `sqlx-local`

- Local SQLite database via `sqlx`
- `reqwest` example that fetches a page and extracts its title and host from a URL
- Minimal `health`, `create`, and `list` endpoints
- OpenAPI docs via `utoipa` and Swagger UI at `/swagger/`
- Keeps the same module split: `config`, `error`, `handler`, `model`, `repository`, `routes`, `service`

### `supabase`

- Supabase connection through `supabase_rs`
- Same URL extraction flow as the local variant
- Minimal `health`, `create`, and `list` endpoints
- OpenAPI docs via `utoipa` and Swagger UI at `/swagger/`
- Same module layout so you can scale it the same way as `lightning-git-backend`

## Why this shape

The backend you pointed to is fairly consistent about a few things:

- State lives in `AppState`
- Route registration is centralized
- Handlers stay thin and hand work off to services or repositories
- Repository functions own persistence details
- Errors are split into small enums with `thiserror`
- Environment variables drive startup

These templates keep that shape, but strip out project-specific behavior like websocket overlays, git mirror logic, and JWT middleware.

## Quick start

### SQLx local

1. Copy `.env.example` to `.env` inside `sqlx-local`
2. Run `cargo run`
3. Create a bookmark:

```bash
curl -X POST http://127.0.0.1:8080/api/bookmarks \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.rust-lang.org"}'
```

4. Open Swagger UI at `http://127.0.0.1:8080/swagger/`

### Supabase

1. Create the table from `supabase/table_creation.sql`
2. Copy `.env.example` to `.env` inside `supabase`
3. Run `cargo run`
4. Open Swagger UI at `http://127.0.0.1:8080/swagger/`

## Suggested extension points

- Add auth middleware to the `/api` scope
- Split DTOs and DB models further as the feature set grows
- Add migrations or table setup for each new resource
- Introduce tests around services and repositories before adding heavier features