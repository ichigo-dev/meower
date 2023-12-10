# Meower

Task management application

![Architecture](./architecture.png)


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
│   ├ system/              ... Contains system components
│   │   ├ assets/          ... Contains common asset files
│   │   │   ├ js/
│   │   │   ├ css/
│   │   │   ├ scss/
│   │   │   ├ locale/
│   │   │   └ email/
│   │   │
│   │   ├ auth_server/     ... Authentication server
│   │   │
│   │   ├ core/            ... Contains core files for the entire application
│   │   │
│   │   ├ datastore/
│   │   │   ├ entity/      ... Database table entities
│   │   │   ├ install/     ... Utility for installing test data and initial data
│   │   │   └ migration/   ... Migration files
│   │   │
│   │   └ service/
│   │       ├ backend/     ... Application backend server
│   │       └ frontend/    ... Application frontend server
│   │
│   ├ .env                 ... Definition of various environment variables
│   ├ .env.example         ... Example of .env
│   └ docker-compose.yml   ... Management docker containers
│
└ spec/                    ... Specifications
```


## Note

- Cargo does not currently support nested workspaces, but we would like to take
  advantage of this.
  ([GitHub Issue](https://github.com/rust-lang/cargo/issues/5042)
