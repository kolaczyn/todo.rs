## How to run

1. Setup rust on your system
1. `cargo install sqlx-cli`
1. `touch db.sql`
1. `sqlx migrate run --source backend/migrations`
1. cargo run backend