# Meower

Task management application


## Description

This application is written in Rust ([1]) for many components. For the frontend,
it uses Sycamore ([2]), a SPA (Single Page Application) framework that uses
WebAssembly. For the backend and authentication server, it uses Axum ([3]), a
web server framework based on Tokio ([4]). It also uses SeaORM ([5]) as a tool
for database migration and model generation.

Casbin ([6]) is used to manage various privileges within the system.

- [1] [Rust](https://www.rust-lang.org)
- [2] [Sycamore](https://sycamore-rs.netlify.app)
- [3] [Axum](https://github.com/tokio-rs/axum)
- [4] [Tokio](https://tokio.rs)
- [5] [SeaORM](https://www.sea-ql.org/SeaORM)
- [6] [Casbin](https://casbin.org)


## Directory structure

In this chapter, we will discuss the configuration files and directories in
this repository. However, we will only list the important components, not
everything.

```
.
├ dev/                         ... Source files
│   ├ docker/                  ... Contains Dockerfiles for containers
│   │   ├ base/                ... Docker base images for service containers
│   │   └ service/             ... Docker images for service containers
│   │
│   ├ proto/                   ... Prototypes
│   │   └ ui_catalog/          ... UI catalog
│   │
│   ├ system/                  ... Contains system components
│   │   ├ api_gw/              ... API gateway
│   │   │   └ core/            ... Server core
│   │   │
│   │   ├ apis/
│   │   │   └ account/         ... Account API
│   │   │       ├ core/        ... Server core
│   │   │       ├ entity/      ... Database table entities
│   │   │       └ migration/   ... Migration files
│   │   │
│   │   ├ app/                 ... Application server
│   │   │   ├ core/            ... Server core
│   │   │   ├ entity/          ... Database table entities
│   │   │   ├ migration/       ... Migration files
│   │   │   └ spa/             ... SPA
│   │   │
│   │   ├ auth/                ... Authentication server
│   │   │   ├ core/            ... Server core
│   │   │   ├ entity/          ... Database table entities
│   │   │   ├ install/         ... Data installer
│   │   │   └ migration/       ... Migration files
│   │   │
│   │   └ common/              ... Utilities
│   │       ├ entity_ext/      ... Entity extensions
│   │       ├ shared/          ... Shared resources
│   │       ├ utility/         ... Utility functions
│   │       └ validator/       ... Validation
│   │
│   ├ .env                     ... Definition of various environment variables
│   ├ .env.example             ... Example of .env
│   └ compose.yml              ... Management docker containers
│
└ spec/                        ... Specifications
```


## Services

- auth: Authentication server.
- api_gw: API Gateway.
- app: Delivery SPA application.
- account: Account API.


## Note

- Cargo does not currently support nested workspaces, but we would like to take
  advantage of this.
  ([GitHub Issue](https://github.com/rust-lang/cargo/issues/5042))
- I want to change the callback URL based on the referrer URL so that the Auth
  server can link IDs with other services. Also, the JWT encryption method
  should be asymmetric key authentication.
