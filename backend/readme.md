### How to setup

Install Postgres on your system, then run the following command:

```shell
cargo install sqlx-cli
sqlx migrate run --source backend/migrations
cargo run --bin backend # or `make back`
```
