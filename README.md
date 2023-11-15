# Meower

Task management application


## Front-end

For front-end development, we use Sycamore, a component-based framework made by Rust and using WebAssembly (wasm).


## Back-end

For back-end development, we use Axum, a Tokio based framework made by Rust.


## Run application

```sh
$ sudo docker-compose build --no-cache
$ sudo docker-compose up
```


## Database

### Migrate database

```sh
$ sudo docker-compose exec migration bash
> sea-orm-cli migrate generate [migration file name]
> sea-orm-cli migrate
> sea-orm-cli generate -o entity/src/entities
```

### Install test data

```sh
$ sudo docker-compose exec backend bash
> cd entity
> cargo run --bin install_test_data
```

### Login postgres

```sh
$ sudo docker-compose exec postgres bash
> psql meower meower
```
