# Database Design


## Causion

- Updates to `updated_at` are performed on the application side. No triggers are used on the database side.


## Table: user

### Columns

| Label         | Name       | Type                        | PK | FK | Unique | Nullable | Default                               | Comment |
| ------------- | ---------- | --------------------------- | -- | -- | ------ | -------- | ------------------------------------- | ------- |
| User ID       | user_id    | bigint                      | o  |    |        | x        | nextval('user_user_id_seq'::regclass) |         |
| Email address | email      | character varying(255)      |    |    | o      | x        |                                       |         |
| Created at    | created_at | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                     |         |
| Updated at    | updated_at | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                     |         |
| Delete flag   | is_deleted | boolean                     |    |    |        |          | false                                 |         |

### Indexes

- "user_pkey" PRIMARY KEY, btree (user_id)
- "user_email_key" UNIQUE CONSTRAINT, btree (email)

### Reference

- TABLE "user_account" CONSTRAINT "user_account_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)
- TABLE "user_auth" CONSTRAINT "user_auth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)


## Table: user_auth

### Columns

| Label         | Name         | Type                        | PK | FK              | Unique | Nullable | Default                                         | Comment                |
| ------------- | ------------ | --------------------------- | -- | --------------- | ------ | -------- | ----------------------------------------------- | ---------------------- |
| User Auth ID  | user_auth_id | bigint                      | o  |                 |        | x        | nextval('user_auth_user_auth_id_seq'::regclass) |                        |
| User ID       | user_id      | bigint                      |    | o user(user_id) |        | x        |                                                 |                        |
| Password      | password     | character varying(255)      |    |                 |        | x        |                                                 | Argon2 hashed password |
| Updated at    | updated_at   | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                               |                        |

### Indexes

- "user_auth_pkey" PRIMARY KEY, btree (user_auth_id)
- "user_auth_user_id_idx" btree (user_id)

### Reference

- "user_auth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)


## Table: user_account

| Label            | Name            | Type                        | PK | FK              | Unique | Nullable | Default                                               | Comment |
| ---------------- | --------------- | --------------------------- | -- | --------------- | ------ | -------- | ----------------------------------------------------- | ------- |
| User Account ID  | user_account_id | bigint                      | o  |                 |        | x        | nextval('user_account_user_account_id_seq'::regclass) |         |
| User ID          | user_id         | bigint                      |    | o user(user_id) |        | x        |                                                       |         |
| Account Name     | account_name    | character varying(255)      |    |                 | o      | x        |                                                       |         |
| Created at       | created_at      | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                                     |         |
| Updated at       | updated_at      | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                                     |         |
| Delete flag      | is_deleted      | boolean                     |    |                 |        |          | false                                                 |         |

### Indexes

- "user_account_pkey" PRIMARY KEY, btree (user_account_id)
- "user_account_account_name_key" UNIQUE CONSTRAINT, btree (account_name)
- "user_account_user_id_idx" btree (user_id)

### Reference

- "user_account_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)
