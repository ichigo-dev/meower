# Database Design


## Table: user

### Columns

| Label         | Name       | Type                        | PK | FK | Unique | Nullable | Default             | Comment |
| ------------- | ---------- | --------------------------- | -- | -- | ------ | -------- | ------------------- | ------- |
| User ID       | user_id    | bigint                      | o  |    | o      | not null | auto increment      |         |
| Email address | email      | character varying           |    |    | o      | not null |                     |         |
| Created at    | created_at | timestamp without time zone |    |    |        | not null |                     |         |
| Updated at    | updated_at | timestamp without time zone |    |    |        | not null | current_timestamp() |         |
| Delete flag   | is_deleted | boolean                     |    |    |        |          | false               |         |

### Indexes

- "user_pkey" PRIMARY KEY, btree (user_id)
- "user_email_key" UNIQUE CONSTRAINT, btree (email)

### Reference

- TABLE "account" CONSTRAINT "fk_account_user_id" FOREIGN KEY (user_id) REFERENCES "user"(user_id)


## Table: user_auth

### Columns

| Label         | Name         | Type                        | PK | FK | Unique | Nullable | Default             | Comment                |
| ------------- | ------------ | --------------------------- | -- | -- | ------ | -------- | ------------------- | ---------------------- |
| User Auth ID  | user_auth_id | bigint                      | o  |    | o      | not null | auto increment      |                        |
| Password      | password     | character varying           |    |    |        | not null |                     | Argon2 hashed password |
| Updated at    | updated_at   | timestamp without time zone |    |    |        | not null | current_timestamp() |                        |
