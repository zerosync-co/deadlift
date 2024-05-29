# Deadlift

Integration solution that utilizes serverless execution of WASM modules to facilitate workflows.

## Thesis

integrations should be:

- platform independent -> able to run anywhere (cloud, on prem, edge, etc)
- embeddable -> able to run within existing services (doesn't require new containers)
- reusable -> able to utilize existing integration modules for new workflows (also enables out of the box solutions)

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
