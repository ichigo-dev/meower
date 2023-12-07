//------------------------------------------------------------------------------
//! UserAccount model.
//------------------------------------------------------------------------------

use meower_core::Validator;

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
    pub is_deleted: bool,
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
        let user_account_name = self.user_account_name.clone().unwrap();

        // Validates fields.
        let mut user_account_name_validator = Validator::new(&user_account_name)
            .not_empty("model_user_account.error.user_account_name.not_empty")
            .min_len(8, "model_user_account.error.user_account_name.min_len")
            .max_len(32, "model_user_account.error.user_account_name.max_len")
            .validate();
        if user_account_name_validator.has_err()
        {
            return Err
            (
                DbErr::Custom(user_account_name_validator.get_first_error())
            );
        }

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        Ok(self)
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
