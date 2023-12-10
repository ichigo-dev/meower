//------------------------------------------------------------------------------
//! UserAccount model.
//------------------------------------------------------------------------------

use meower_core::{ Validator, I18n };
use crate::Validate;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_account")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_account_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub user_account_name: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_logined_at: DateTime,
    pub is_deleted: bool,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user_account by user account name.
    //--------------------------------------------------------------------------
    pub fn find_by_user_account_name( user_account_name: &str ) -> Select<Self>
    {
        Self::find().filter(Column::UserAccountName.eq(user_account_name))
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
#[async_trait]
impl ActiveModelBehavior for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Before save.
    //--------------------------------------------------------------------------
    async fn before_save<C>
    (
        mut self,
        _hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
            self.set(Column::LastLoginedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        Ok(self)
    }

}

#[async_trait]
impl Validate for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>
    (
        &self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let user_account_name = self.user_account_name.clone().unwrap();

        // Checks if the account already exists.
        if self.user_id.is_set() == false
        {
            if Entity::find_by_user_account_name(&user_account_name)
                .one(hdb)
                .await
                .unwrap()
                .is_some()
            {
                return Err(i18n.get("model_user_account.error.user_account_name.already_exists"));
            }
        }

        // Validates fields.
        let mut user_account_name_validator = Validator::new(&user_account_name)
            .not_empty
            (
                &i18n.get
                (
                    "model_user_account.error.user_account_name.not_empty"
                )
            )
            .min_len
            (
                3,
                &i18n.get_with
                (
                    "model_user_account.error.user_account_name.min_len",
                    [("min_len", "3")].into()
                )
            )
            .max_len
            (
                32,
                &i18n.get_with
                (
                    "model_user_account.error.user_account_name.max_len",
                    [("max_len", "32")].into()
                )
            )
            .regex
            (
                r"^[a-zA-Z0-9_]+$",
                &i18n.get("model_user_account.error.user_account_name.regex")
            )
            .validate();
        if user_account_name_validator.has_err()
        {
            return Err(user_account_name_validator.get_first_error());
        }

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::organization_member::Entity")]
    OrganizationMember,

    #[sea_orm(has_many = "super::project_member::Entity")]
    ProjectMember,

    #[sea_orm(has_many = "super::task::Entity")]
    Task,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMember,
}

impl Related<super::organization_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationMember.def()
    }
}

impl Related<super::project_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMember.def()
    }
}

impl Related<super::task::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Task.def()
    }
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}

impl Related<super::workspace_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMember.def()
    }
}
