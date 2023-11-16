# Meower

Task management application


## Application Components

### Front-end

For front-end development, we use Sycamore, a component-based framework made by Rust and using WebAssembly (wasm).

### Back-end

For back-end development, we use Axum, a Tokio based framework made by Rust.

### Migration

- [/migration](./migration/README.md)

### Run application

```sh
$ sudo docker-compose build --no-cache
$ sudo docker-compose up
```


## PostgreSQL Database

### Login postgres

```sh
$ sudo docker-compose exec postgres bash
> psql meower meower
```
