# public.project_member_authority

## Description

Project member authority

## Columns

| Name                        | Type         | Default                                                                       | Nullable | Children                                          | Parents | Comment                     |
| --------------------------- | ------------ | ----------------------------------------------------------------------------- | -------- | ------------------------------------------------- | ------- | --------------------------- |
| project_member_authority_id | bigint       | nextval('project_member_authority_project_member_authority_id_seq'::regclass) | false    | [public.project_member](public.project_member.md) |         | Project member authority ID |
| symbol                      | varchar(255) |                                                                               | false    |                                                   |         | Symbol                      |
| value                       | integer      |                                                                               | false    |                                                   |         | Value                       |

## Constraints

| Name                          | Type        | Definition                                |
| ----------------------------- | ----------- | ----------------------------------------- |
| project_member_authority_pkey | PRIMARY KEY | PRIMARY KEY (project_member_authority_id) |

## Indexes

| Name                          | Definition                                                                                                                     |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| project_member_authority_pkey | CREATE UNIQUE INDEX project_member_authority_pkey ON public.project_member_authority USING btree (project_member_authority_id) |

## Relations

![er](public.project_member_authority.svg)

---

> Generated by [tbls](https://github.com/k1LoW/tbls)
