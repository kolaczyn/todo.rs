## How to run

- 1\. Setup rust on your system
- 2\. Setup backend:
  - 1\. `cargo install sqlx-cli`
  - 2\. `touch db.sql`
  - 3\. `sqlx migrate run --source backend/migrations`
  - 4\. `cargo run --bin backend`
- 3\. Setup frontend:
  - 1\. `rustup target add wasm32-unknown-unknown`
  - 2\. `cargo install --locked trunk`
  - 3\. Allow CORS via browser extension (for now, I'll have to fix this later :p)
  - 4\. `cd fronted && trunk serve --port 8000`