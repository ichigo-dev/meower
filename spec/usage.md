# Usage

Run these commands inside the `/dev` directory.


## Initialize project

```sh
$ make init
```


## Run application

```sh
$ make up
$ make logs
```


## Migrate database

```sh
$ make dev
> cd /system/datastore
> sea-orm-cli migrate generate [migration file name]
> sea-orm-cli migrate
> sea-orm-cli generate -o entity/src/entities
```


## Install test data

```sh
$ make install bin=install_test_data
```


## Convert SCSS to CSS

```sh
$ make sass
```


## Login postgres

```sh
$ make db
> psql meower meower
```
