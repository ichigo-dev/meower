# public.user_account

## Description

## Columns

| Name              | Type                        | Default                                               | Nullable | Children                                                                                                                                                                                          | Parents                       | Comment |
| ----------------- | --------------------------- | ----------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- | ------- |
| user_account_id   | bigint                      | nextval('user_account_user_account_id_seq'::regclass) | false    | [public.organization_member](public.organization_member.md) [public.workspace_member](public.workspace_member.md) [public.project_member](public.project_member.md) [public.task](public.task.md) |                               |         |
| user_id           | bigint                      |                                                       | false    |                                                                                                                                                                                                   | [public.user](public.user.md) |         |
| user_account_name | varchar(255)                |                                                       | false    |                                                                                                                                                                                                   |                               |         |
| display_name      | varchar(255)                |                                                       | false    |                                                                                                                                                                                                   |                               |         |
| created_at        | timestamp without time zone | CURRENT_TIMESTAMP                                     | false    |                                                                                                                                                                                                   |                               |         |
| updated_at        | timestamp without time zone | CURRENT_TIMESTAMP                                     | false    |                                                                                                                                                                                                   |                               |         |
| is_deleted        | boolean                     | false                                                 | false    |                                                                                                                                                                                                   |                               |         |

## Constraints

| Name                               | Type        | Definition                                       |
| ---------------------------------- | ----------- | ------------------------------------------------ |
| user_account_user_id_fkey          | FOREIGN KEY | FOREIGN KEY (user_id) REFERENCES "user"(user_id) |
| user_account_pkey                  | PRIMARY KEY | PRIMARY KEY (user_account_id)                    |
| user_account_user_account_name_key | UNIQUE      | UNIQUE (user_account_name)                       |

## Indexes

| Name                               | Definition                                                                                                    |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| user_account_pkey                  | CREATE UNIQUE INDEX user_account_pkey ON public.user_account USING btree (user_account_id)                    |
| user_account_user_account_name_key | CREATE UNIQUE INDEX user_account_user_account_name_key ON public.user_account USING btree (user_account_name) |
| user_account_user_id_idx           | CREATE INDEX user_account_user_id_idx ON public.user_account USING btree (user_id)                            |

## Relations

```mermaid
erDiagram

"public.organization_member" }o--|| "public.user_account" : "FOREIGN KEY (user_account_id) REFERENCES user_account(user_account_id)"
"public.workspace_member" }o--|| "public.user_account" : "FOREIGN KEY (user_account_id) REFERENCES user_account(user_account_id)"
"public.project_member" }o--|| "public.user_account" : "FOREIGN KEY (user_account_id) REFERENCES user_account(user_account_id)"
"public.task" }o--|| "public.user_account" : "FOREIGN KEY (owner_user_account_id) REFERENCES user_account(user_account_id)"
"public.user_account" }o--|| "public.user" : "FOREIGN KEY (user_id) REFERENCES "user"(user_id)"

"public.user_account" {
  bigint user_account_id
  bigint user_id FK
  varchar_255_ user_account_name
  varchar_255_ display_name
  timestamp_without_time_zone created_at
  timestamp_without_time_zone updated_at
  boolean is_deleted
}
"public.organization_member" {
  bigint organization_member_id
  bigint organization_id FK
  bigint user_account_id FK
  bigint organization_member_authority_id FK
}
"public.workspace_member" {
  bigint workspace_member_id
  bigint workspace_id FK
  bigint user_account_id FK
  bigint workspace_member_authority_id FK
}
"public.project_member" {
  bigint project_member_id
  bigint project_id FK
  bigint user_account_id FK
  bigint project_member_authority_id FK
}
"public.task" {
  bigint task_id
  bigint project_id FK
  bigint owner_user_account_id FK
  varchar_255_ title
  text content
  timestamp_without_time_zone created_at
  timestamp_without_time_zone updated_at
  boolean is_deleted
}
"public.user" {
  bigint user_id
  varchar_255_ email
  timestamp_without_time_zone created_at
  timestamp_without_time_zone updated_at
  boolean is_deleted
}
```

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
