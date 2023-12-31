# Meower

Task management application

![Sequence - Auth](./sequence_auth.png)


## Description

This application is written in Rust ([1]) for many components. For the frontend,
it uses Sycamore ([2]), a SPA (Single Page Application) framework that uses
WebAssembly. For the backend and authentication server, it uses Axum ([3]), a
web server framework based on Tokio ([4]). It also uses SeaORM ([5]) as a tool
for database migration and model generation.

- [1] [Rust](https://www.rust-lang.org)
- [2] [Sycamore](https://sycamore-rs.netlify.app)
- [3] [Axum](https://github.com/tokio-rs/axum)
- [4] [Tokio](https://tokio.rs)
- [5] [SeaORM](https://www.sea-ql.org/SeaORM)


## Directory structure

In this chapter, we will discuss the configuration files and directories in
this repository. However, we will only list the important components, not
everything.

```
.
├ dev/                     ... Source files
│   ├ docker/              ... Contains Dockerfiles for containers
│   │   ├ base/            ... Docker base images for service containers
│   │   └ service/         ... Docker images for service containers
│   │
│   ├ proto/               ... Prototypes
│   │   └ ui_catalog/      ... UI catalog
│   │
│   ├ system/              ... Contains system components
│   │   ├ util/            ... Contains utility files for the entire application
│   │   │   ├ type/        ... Definition of common types
│   │   │   └ validator/   ... Validation utility
│   │   │
│   │   ├ database/
│   │   │   ├ entity/      ... Database table entities
│   │   │   ├ install/     ... Utility for installing test data and initial data
│   │   │   └ migration/   ... Migration files
│   │   │
│   │   ├ layer/
│   │   │   └ auth/        ... Authentication layer
│   │   │
│   │   └ service/
│   │       ├ schema/      ... API schemas
│   │       ├ api/         ... Application backend(API) server
│   │       └ spa/         ... Application frontend(SPA) server
│   │
│   ├ .env                 ... Definition of various environment variables
│   ├ .env.example         ... Example of .env
│   └ compose.yml          ... Management docker containers
│
└ spec/                    ... Specifications
```


## Note

- Cargo does not currently support nested workspaces, but we would like to take
  advantage of this.
  ([GitHub Issue](https://github.com/rust-lang/cargo/issues/5042)
