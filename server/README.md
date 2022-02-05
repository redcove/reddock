# Red Dock API Server
Refrence notes for developing the red dock api server.

## Routes
All route handlers are kept in `src/routes/<handler name>`.

At the top of the file are the representations that are stored in the database.

Next each function create, delete, list, ect... have any specific json struct need about it then the function itself.

## Errors
For errors we log the actual error using tracing and then return a generic `axum::http::StatusCode`. This makes it easy to just use `map_err` as part of a chain.

```rust
    <do stuff>
    .await
    .map_err(|e| {
        tracing::error!("Failed to create report in db: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .<anthing else>;
```

## SQLX
Make sure to export the Database url so that the sqlx `query!` macro works.
```
export DATABASE_URL="sqlite:reddock.db"
```
### CLI
```
cargo install sqlx-cli
```

### Create db
```
splx datebase create
```

### Migrations
When creating migrations use reversable migrations:
```
sqlx migrate add -r <name>
```
