# Meower migration

Data migration and install.


## Migrate database

```sh
$ sudo docker-compose exec migration bash
> sea-orm-cli migrate generate [migration file name]
> sea-orm-cli migrate
> sea-orm-cli generate -o entity/src/entities
```


## Install test data

```sh
$ sudo docker-compose exec backend bash
> cd entity
> cargo run --bin install_test_data
```
