### How to setup

```shell
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
# Allow CORS via browser extension (for now, I'll have to fix this later :p)
cd frontend && trunk serve --port 8000
```
