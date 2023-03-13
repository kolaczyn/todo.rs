### How to setup

1. Setup rust
1. Setup Postgres
1. Run the following commands (all of them in the `backend/` directory)

```shell
cargo install sqlx-cli
sqlx migrate run
cargo run --backend
```
