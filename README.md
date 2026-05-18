# repro

## To reproduce the bug

1. Run the server (cargo run)
2. Run the repro (node repro.js)

```
thread 'main:worker:1' (4181730) panicked at /xxx/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ntex-http-1.2.0/src/value.rs:491:54:
called `Result::unwrap()` on an `Err` value: InvalidHeaderValue
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## It does not happen with docker

```
docker run \
  -p 4000:4000 \
  -v ./supergraph.graphql:/app/supergraph.graphql \
  -v ./router.config.yaml:/app/router.config.yaml \
  ghcr.io/graphql-hive/router:latest
```
