# public.group

## Description

Group table

## Columns

| Name           | Type                        | Default                                 | Nullable | Children                                                                                                                                                                                    | Parents | Comment        |
| -------------- | --------------------------- | --------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- | -------------- |
| group_id       | bigint                      | nextval('group_group_id_seq'::regclass) | false    | [public.group_member](public.group_member.md) [public.group_workspace](public.group_workspace.md) [public.group_avatar](public.group_avatar.md) [public.group_cover](public.group_cover.md) |         | Group ID       |
| group_name     | varchar(255)                |                                         | false    |                                                                                                                                                                                             |         | Group name     |
| name           | varchar(255)                |                                         | false    |                                                                                                                                                                                             |         | Name           |
| description    | text                        |                                         | true     |                                                                                                                                                                                             |         | Description    |
| representative | varchar(255)                |                                         | true     |                                                                                                                                                                                             |         | Representative |
| location       | varchar(255)                |                                         | true     |                                                                                                                                                                                             |         | Location       |
| email          | varchar(255)                |                                         | true     |                                                                                                                                                                                             |         | Email          |
| telno          | varchar(255)                |                                         | true     |                                                                                                                                                                                             |         | Telno          |
| founded_at     | timestamp without time zone | CURRENT_TIMESTAMP                       | true     |                                                                                                                                                                                             |         | Founded date   |
| created_at     | timestamp without time zone | CURRENT_TIMESTAMP                       | false    |                                                                                                                                                                                             |         | Create date    |
| updated_at     | timestamp without time zone | CURRENT_TIMESTAMP                       | false    |                                                                                                                                                                                             |         | Update date    |
| is_public      | boolean                     |                                         | false    |                                                                                                                                                                                             |         | Is public      |

## Constraints

| Name                 | Type        | Definition             |
| -------------------- | ----------- | ---------------------- |
| group_pkey           | PRIMARY KEY | PRIMARY KEY (group_id) |
| group_group_name_key | UNIQUE      | UNIQUE (group_name)    |

## Indexes

| Name                 | Definition                                                                          |
| -------------------- | ----------------------------------------------------------------------------------- |
| group_pkey           | CREATE UNIQUE INDEX group_pkey ON public."group" USING btree (group_id)             |
| group_group_name_key | CREATE UNIQUE INDEX group_group_name_key ON public."group" USING btree (group_name) |

## Relations

![er](public.group.svg)

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
