# Deadlift

Integration solution that utilizes serverless execution of WASM modules to facilitate workflows.

## Thesis

integrations should be:

- platform independent -> able to run anywhere (cloud, on prem, edge, etc)
- embeddable -> able to run within existing services (doesn't require new containers)
- reusable -> able to utilize existing integration modules for new workflows (also enables out of the box solutions)

## Quickstart

Install the [rust toolchain](https://www.rust-lang.org/tools/install)

### Running the web service

`cargo run -p deadlift-service`

### Creating a module

1. Install the wasm32-wasi target
   `rustup target add wasm32-wasi`

2. Create a rust lib project
   `cargo new <module name> --lib`

3. Add the following configuration to the project `Cargo.toml`

```
[lib]
crate-type = ["cdylib"]
```

4. Write your module! Examples can be found [here](./examples/calculator/add_ten)

### Run an example

Running the calculator example

1. Run the web service

```
cargo run -p deadlift-service
```

2. Run the example

```
cargo run --example calculator
```

## How it works

### Web Assembly

WASM modules are executed such that they can be embedded within any environment in a secure and performant way

### NATS

NATS messaging is used to execute modules asynchronously with builtin retry mechanisms

## POC

Goal: do series of calculations, where the calculations are the workflow

- actix-web application
- no auth/users/identity
- embedded sqlite db
- synchronous module execution
- no NATS
- no docker

routes:

- POST /modules
- GET /modules
- POST /modules/{id}/execute

- POST /workflows
- GET /workflows
- POST /workflows/{id}/execute
