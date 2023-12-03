# Database Design


## Development memo

- Separate organization_member(authority) into organization_authority table
- Separate workspace_member(authority) into workspace_authority table
- Separate project_member(authority) into project_authority table
- Create a role table that represents authority for organization_authority, workspace_authority and project_authority


## Causion

- Updates to `updated_at` are performed on the application side. No triggers are used on the database side.


## Table: user

### Columns

| Label         | Name       | Type                        | PK | FK | Unique | Nullable | Default                               | Comment |
| ------------- | ---------- | --------------------------- | -- | -- | ------ | -------- | ------------------------------------- | ------- |
| User ID       | user_id    | bigint                      | o  |    |        | x        | nextval('user_user_id_seq'::regclass) |         |
| Email address | email      | character varying(255)      |    |    | o      | x        |                                       |         |
| Create date   | created_at | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                     |         |
| Update date   | updated_at | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                     |         |
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
| User auth ID  | user_auth_id | bigint                      | o  |                 |        | x        | nextval('user_auth_user_auth_id_seq'::regclass) |                        |
| User ID       | user_id      | bigint                      |    | o user(user_id) |        | x        |                                                 |                        |
| Password      | password     | character varying(255)      |    |                 |        | x        |                                                 | Argon2 hashed password |
| Update date   | updated_at   | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                               |                        |

### Indexes

- "user_auth_pkey" PRIMARY KEY, btree (user_auth_id)
- "user_auth_user_id_idx" btree (user_id)

### Reference

- "user_auth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)


## Table: user_account

| Label             | Name              | Type                        | PK | FK              | Unique | Nullable | Default                                               | Comment                                              |
| ----------------- | ----------------- | --------------------------- | -- | --------------- | ------ | -------- | ----------------------------------------------------- | ---------------------------------------------------- |
| User account ID   | user_account_id   | bigint                      | o  |                 |        | x        | nextval('user_account_user_account_id_seq'::regclass) |                                                      |
| User ID           | user_id           | bigint                      |    | o user(user_id) |        | x        |                                                       |                                                      |
| User account name | user_account_name | character varying(255)      |    |                 | o      | x        |                                                       | Only alphabets, numbers, and underscores can be used |
| Display name      | display_name      | character varying(255)      |    |                 |        | x        |                                                       |                                                      |
| Create date       | created_at        | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                                     |                                                      |
| Update date       | updated_at        | timestamp without time zone |    |                 |        | x        | CURRENT_TIMESTAMP                                     |                                                      |
| Delete flag       | is_deleted        | boolean                     |    |                 |        |          | false                                                 |                                                      |

### Indexes

- "user_account_pkey" PRIMARY KEY, btree (user_account_id)
- "user_account_user_account_name_key" UNIQUE CONSTRAINT, btree (user_account_name)
- "user_account_user_id_idx" btree (user_id)

### Reference

- "user_account_user_id_fkey" FOREIGN KEY (user_id) REFERENCES "user"(user_id)
- TABLE "organization_member" CONSTRAINT "organization_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)
- TABLE "workspace_member" CONSTRAINT "workspace_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)
- TABLE "project_member" CONSTRAINT "project_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)
- TABLE "task" CONSTRAINT "task_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)


## Table: temporary_user

| Label             | Name              | Type                        | PK | FK | Unique | Nullable | Default                                                   | Comment                |
| ----------------- | ----------------- | --------------------------- | -- | -- | ------ | -------- | --------------------------------------------------------- | ---------------------- |
| Temporary User ID | temporary_user_id | bigint                      | o  |    |        | x        | nextval('temporary_user_temporary_user_id_seq'::regclass) |                        |
| Email address     | email             | character varying(255)      |    |    | o      | x        |                                                           |                        |
| Password          | password          | character varying(255)      |    |    |        | x        |                                                           | Argon2 hashed password |
| Create date       | created_at        | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                                         |                        |

### Indexes

- "temporary_user_pkey" PRIMARY KEY, btree (temporary_user_id)
- "temporary_user_email_key" UNIQUE CONSTRAINT, btree (email)

### Reference

- TABLE "temporary_user_token" CONSTRAINT "temporary_user_token_temporary_user_id_fkey" FOREIGN KEY (temporary_user_id) REFERENCES "temporary_user"(temporary_user_id)


## Table: temporary_user_token

| Label                   | Name                    | Type                        | PK | FK                                  | Unique | Nullable | Default                                                               | Comment |
| ----------------------- | ----------------------- | --------------------------- | -- | ----------------------------------- | ------ | -------- | --------------------------------------------------------------------- | ------- |
| Temporary User Token ID | temporary_user_token_id | bigint                      | o  |                                     |        | x        | nextval('temporary_user_token_temporary_user_token_id_seq'::regclass) |         |
| Temporary User ID       | temporary_user_id       | bigint                      |    | o temporary_user(temporary_user_id) | o      | x        |                                                                       |         |
| Token                   | token                   | character varying(255)      |    |                                     |        | x        |                                                                       |         |
| Create date             | created_at              | timestamp without time zone |    |                                     |        | x        | CURRENT_TIMESTAMP                                                     |         |

### Indexes

- "temporary_user_token_pkey" PRIMARY KEY, btree (temporary_user_token_id)
- "temporary_user_token_token_key", btree (token)

### Reference

- "temporary_user_token_temporary_user_id_fkey" FOREIGN KEY (temporary_user_id) REFERENCES "temporary_user"(temporary_user_id)


## Table: organization

| Label             | Name              | Type                        | PK | FK | Unique | Nullable | Default                                               | Comment                                              |
| ----------------- | ----------------- | --------------------------- | -- | -- | ------ | -------- | ----------------------------------------------------- | ---------------------------------------------------- |
| Organization ID   | organization_id   | bigint                      | o  |    |        | x        | nextval('organization_organization_id_seq'::regclass) |                                                      |
| Organization Name | organization_name | character varying(255)      |    |    | o      | x        |                                                       | Only alphabets, numbers, and underscores can be used |
| Display Name      | display_name      | character varying(255)      |    |    |        | x        |                                                       |                                                      |
| Create date       | created_at        | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                                     |                                                      |
| Update date       | updated_at        | timestamp without time zone |    |    |        | x        | CURRENT_TIMESTAMP                                     |                                                      |
| Delete flag       | is_deleted        | boolean                     |    |    |        |          | false                                                 |                                                      |

### Indexes

- "organization_pkey" PRIMARY KEY, btree (organization_id)
- "organization_organization_name_key" UNIQUE CONSTRAINT, btree (organization_name)

### Reference

- TABLE "organization_member" CONSTRAINT "organization_member_organization_id_fkey" FOREIGN KEY (organization_id) REFERENCES "organization"(organization_id)
- TABLE "workspace" CONSTRAINT "workspace_organization_id_fkey" FOREIGN KEY (organization_id) REFERENCES "organization"(organization_id)


## Table: organization_member

| Label                  | Name                   | Type    | PK | FK                              | Unique | Nullable | Default                                                             | Comment                |
| ---------------------- | ---------------------- | ------- | -- | ------------------------------- | ------ | -------- | ------------------------------------------------------------------- | ---------------------- |
| Organization member ID | organization_member_id | bigint  | o  |                                 |        | x        | nextval('organization_member_organization_member_id_seq'::regclass) |                        |
| Organization ID        | organization_id        | bigint  |    | o organization(organization_id) |        | x        |                                                                     |                        |
| User account ID        | user_account_id        | bigint  |    | o user_account(user_account_id) |        | x        |                                                                     |                        |
| Authority              | authority              | tinyint |    |                                 |        | x        |                                                                     | (1: member, 99: admin) |

### Indexes

- "organization_member_organization_id_idx" btree (organization_id)
- "organization_member_user_account_id_idx" btree (user_account_id)

### Reference

- "organization_member_organization_id_fkey" FOREIGN KEY (organization_id) REFERENCES "organization"(organization_id)
- "organization_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)


## Table: workspace

| Label           | Name            | Type                        | PK | FK                              | Unique | Nullable | Default                                         | Comment                                              |
| --------------- | --------------- | --------------------------- | -- | ------------------------------- | ------ | -------- | ----------------------------------------------- | ---------------------------------------------------- |
| Workspace ID    | workspace_id    | bigint                      | o  |                                 |        | x        | nextval('workspace_workspace_id_seq'::regclass) |                                                      |
| Workspace Name  | workspace_name  | character varying(255)      |    |                                 | o      | x        |                                                 | Only alphabets, numbers, and underscores can be used |
| Display Name    | display_name    | character varying(255)      |    |                                 |        | x        |                                                 |                                                      |
| Organization ID | organization_id | bigint                      |    | o organization(organization_id) |        | x        |                                                 |                                                      |
| Create date     | created_at      | timestamp without time zone |    |                                 |        | x        | CURRENT_TIMESTAMP                               |                                                      |
| Update date     | updated_at      | timestamp without time zone |    |                                 |        | x        | CURRENT_TIMESTAMP                               |                                                      |
| Delete flag     | is_deleted      | boolean                     |    |                                 |        |          | false                                           |                                                      |

### Indexes

- "workspace_organization_id_idx" btree (organization_id)

### Reference

- "workspace_organization_id_fkey" FOREIGN KEY (organization_id) REFERENCES "organization"(organization_id)
- TABLE "workspace_member" CONSTRAINT "workspace_member_workspace_id_fkey" FOREIGN KEY (workspace_id) REFERENCES "workspace"(workspace_id)
- TABLE "project" CONSTRAINT "project_workspace_id_fkey" FOREIGN KEY (workspace_id) REFERENCES "workspace"(workspace_id)


## Table: workspace_member

| Label               | Name                | Type    | PK | FK                              | Unique | Nullable | Default                                                       | Comment                |
| ------------------- | ------------------- | ------- | -- | ------------------------------- | ------ | -------- | ------------------------------------------------------------- | ---------------------- |
| Workspace member ID | workspace_member_id | bigint  | o  |                                 |        | x        | nextval('workspace_member_workspace_member_id_seq'::regclass) |                        |
| Workspace ID        | workspace_id        | bigint  |    | o workspace(workspace_id)       |        | x        |                                                               |                        |
| User account ID     | user_account_id     | bigint  |    | o user_account(user_account_id) |        | x        |                                                               |                        |
| Authority           | authority           | tinyint |    |                                 |        | x        |                                                               | (1: member, 99: admin) |

### Indexes

- "workspace_member_workspace_id_idx" btree (workspace_id)
- "workspace_member_user_account_id_idx" btree (user_account_id)

### Reference

- "workspace_member_workspace_id_fkey" FOREIGN KEY (workspace_id) REFERENCES "workspace"(workspace_id)
- "workspace_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)


## Table: project

| Label        | Name         | Type                        | PK | FK                        | Unique | Nullable | Default                                     | Comment                                              |
| ------------ | ------------ | --------------------------- | -- | ------------------------- | ------ | -------- | ------------------------------------------- | ---------------------------------------------------- |
| Project ID   | project_id   | bigint                      | o  |                           |        | x        | nextval('project_project_id_seq'::regclass) |                                                      |
| Project Name | project_name | character varying(255)      |    |                           | o      | x        |                                             | Only alphabets, numbers, and underscores can be used |
| Display Name | display_name | character varying(255)      |    |                           |        | x        |                                             |                                                      |
| Workspace ID | workspace_id | bigint                      | o  | o workspace(workspace_id) |        | x        |                                             |                                                      |
| Create date  | created_at   | timestamp without time zone |    |                           |        | x        | CURRENT_TIMESTAMP                           |                                                      |
| Update date  | updated_at   | timestamp without time zone |    |                           |        | x        | CURRENT_TIMESTAMP                           |                                                      |
| Delete flag  | is_deleted   | boolean                     |    |                           |        |          | false                                       |                                                      |

### Indexes

- "project_workspace_id_idx" btree (workspace_id)

### Reference

- "project_workspace_id_fkey" FOREIGN KEY (workspace_id) REFERENCES "workspace"(workspace_id)
- TABLE "project_member" CONSTRAINT "project_member_project_id_fkey" FOREIGN KEY (project_id) REFERENCES "project"(project_id)
- TABLE "task" CONSTRAINT "task_project_id_fkey" FOREIGN KEY (project_id) REFERENCES "project"(project_id)


## Table: project_member

| Label             | Name              | Type    | PK | FK                              | Unique | Nullable | Default                                                   | Comment                |
| ----------------- | ----------------- | ------- | -- | ------------------------------- | ------ | -------- | --------------------------------------------------------- | ---------------------- |
| Project member ID | project_member_id | bigint  | o  |                                 |        | x        | nextval('project_member_project_member_id_seq'::regclass) |                        |
| Project ID        | project_id        | bigint  |    | o project(project_id)           |        | x        |                                                           |                        |
| User account ID   | user_account_id   | bigint  |    | o user_account(user_account_id) |        | x        |                                                           |                        |
| Authority         | authority         | tinyint |    |                                 |        | x        |                                                           | (1: member, 99: admin) |

### Indexes

- "project_member_project_id_idx" btree (project_id)
- "project_member_user_account_id_idx" btree (user_account_id)

### Reference

- "project_member_project_id_fkey" FOREIGN KEY (project_id) REFERENCES "project"(project_id)
- "project_member_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)


## Table: Task

| Label                 | Name                  | Type                        | PK | FK                              | Unique | Nullable | Default                               | Comment |
| --------------------- | --------------------- | --------------------------- | -- | ------------------------------- | ------ | -------- | ------------------------------------- | ------- |
| Task ID               | task_id               | bigint                      | o  |                                 |        | x        | nextval('task_task_id_seq'::regclass) |         |
| Project ID            | project_id            | bigint                      |    | o project(project_id)           |        | x        |                                       |         |
| Owner user account ID | owner_user_account_id | bigint                      |    | o user_account(user_account_id) |        | x        |                                       |         |
| Title                 | title                 | character varying(255)      |    |                                 |        | x        |                                       |         |
| Content               | content               | text                        |    |                                 |        |          |                                       |         |
| Create date           | created_at            | timestamp without time zone |    |                                 |        | x        | CURRENT_TIMESTAMP                     |         |
| Update date           | updated_at            | timestamp without time zone |    |                                 |        | x        | CURRENT_TIMESTAMP                     |         |
| Delete flag           | is_deleted            | boolean                     |    |                                 |        |          | false                                 |         |

### Indexes

- "task_project_id_idx" btree (project_id)
- "task_owner_user_account_id_idx" btree (owner_user_account_id)

### Reference

- "task_project_id_fkey" FOREIGN KEY (project_id) REFERENCES "project"(project_id)
- "task_owner_user_account_id_fkey" FOREIGN KEY (user_account_id) REFERENCES "user_account"(user_account_id)
