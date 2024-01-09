Follow the documentation [here](https://bcnrust.github.io/devbcn-workshop/index.html)
<br />
<br />
Video resume [here](https://youtu.be/DCpILwGas-M?t=6322)
<br />
Docs resume [here](https://bcnrust.github.io/devbcn-workshop/backend/16_films_endpoints.html)

## Cargo.toml
Changel sqlx <b>features</b> from `"runtime-actix-native-tls"` to `"runtime-tokio-native-tls"`

## Run applicaton
```bash
cargo watch -qcx 'shuttle run'
```

### Shuttle command lists
- `cargo shuttle deploy`: Deploy the project to the cloud.
- `cargo shuttle logs`: Display the logs of a deployment.
- `cargo shuttle status`: Display the status of the service.
- `cargo shuttle project status`: Display the status of the project.
- `cargo shuttle project list`: Display a list of projects and their current status.
- `cargo shuttle project restart`: Restart a project. Useful when you need to upgrade the version of your Shuttle dependencies.
- `cargo shuttle resource list`: Display a list of resources and their current status. Useful to see connection strings and other information about the resources used by the project.