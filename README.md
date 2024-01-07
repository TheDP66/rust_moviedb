Follow the documentation [here](https://bcnrust.github.io/devbcn-workshop/index.html)

## Version lock
Copy this version package to `api > shuttle > Cargo.toml`
```bash
shuttle-actix-web = "0.21.0"
shuttle-runtime = "0.21.0"
shuttle-shared-db = { version = "0.21.0", features = ["postgres"] }
sqlx = { version = "0.6.3", default-features = false, features = [
    "runtime-actix-native-tls",
    "macros",
    "postgres",
    "uuid",
    "chrono",
    "json",
] }
```

## Run applicaton
```bash
cargo shuttle run
```

## Deploy to shuttle
```bash
cargo shuttle deploy
```