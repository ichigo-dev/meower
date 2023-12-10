# public.user_account

## Description

User account table

## Columns

| Name              | Type                        | Default                                               | Nullable | Children                                                                                                                                                                                                                                      | Parents                       | Comment           |
| ----------------- | --------------------------- | ----------------------------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- | ----------------- |
| user_account_id   | bigint                      | nextval('user_account_user_account_id_seq'::regclass) | false    | [public.organization_member](public.organization_member.md) [public.workspace_member](public.workspace_member.md) [public.project_member](public.project_member.md) [public.task](public.task.md) [public.task_member](public.task_member.md) |                               | User account ID   |
| user_id           | bigint                      |                                                       | false    |                                                                                                                                                                                                                                               | [public.user](public.user.md) | User ID           |
| user_account_name | varchar(255)                |                                                       | false    |                                                                                                                                                                                                                                               |                               | User account name |
| display_name      | varchar(255)                |                                                       | false    |                                                                                                                                                                                                                                               |                               | Display name      |
| last_logined_at   | timestamp without time zone | CURRENT_TIMESTAMP                                     | false    |                                                                                                                                                                                                                                               |                               |                   |
| created_at        | timestamp without time zone | CURRENT_TIMESTAMP                                     | false    |                                                                                                                                                                                                                                               |                               | Create date       |
| updated_at        | timestamp without time zone | CURRENT_TIMESTAMP                                     | false    |                                                                                                                                                                                                                                               |                               | Update date       |
| is_deleted        | boolean                     | false                                                 | false    |                                                                                                                                                                                                                                               |                               | Soft delete flag  |

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

![er](public.user_account.svg)

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
