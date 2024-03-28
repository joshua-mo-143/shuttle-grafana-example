# Shuttle Grafana example
This repository serves as an example of how you can set up sending logs to a Grafana Loki instance with Shuttle.

## Deployment
To deploy, run the following (requires `cargo-shuttle` installed):
```bash
cargo shuttle init --from joshua-mo-143/shuttle-grafana-example
```
Follow the prompt. Next, add your secrets to `Secrets.toml` (see the `Secrets.example.toml` file).

Then run `cargo shuttle deploy --allow-dirty`!
