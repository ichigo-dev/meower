//------------------------------------------------------------------------------
//! Workspace model.
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
#[sea_orm(table_name = "workspace")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub workspace_id: i64,
    #[sea_orm(unique)]
    pub workspace_name: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds workspace by workspace name.
    //--------------------------------------------------------------------------
    pub fn find_by_workspace_name( workspace_name: &str ) -> Select<Self>
    {
        Self::find().filter(Column::WorkspaceName.eq(workspace_name))
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
        let workspace_name = self.workspace_name.clone().unwrap();

        // Checks if the account already exists.
        if self.workspace_id.is_set() == false
        {
            if Entity::find_by_workspace_name(&workspace_name)
                .one(hdb)
                .await
                .unwrap()
                .is_some()
            {
                return Err(i18n.get("model_workspace.error.workspace_name.already_exists"));
            }
        }

        // Validates fields.
        let mut workspace_name_validator = Validator::new(&workspace_name)
            .not_empty
            (
                &i18n.get
                (
                    "model_workspace.error.workspace_name.not_empty"
                )
            )
            .min_len
            (
                3,
                &i18n.get_with
                (
                    "model_workspace.error.workspace_name.min_len",
                    [("min_len", "3")].into()
                )
            )
            .max_len
            (
                32,
                &i18n.get_with
                (
                    "model_workspace.error.workspace_name.max_len",
                    [("max_len", "32")].into()
                )
            )
            .regex
            (
                r"^[a-zA-Z0-9_]+$",
                &i18n.get("model_workspace.error.workspace_name.regex")
            )
            .validate();
        if workspace_name_validator.has_err()
        {
            return Err(workspace_name_validator.get_first_error());
        }

        let display_name = self.display_name.clone().unwrap();
        let mut display_name_validator = Validator::new(&display_name)
            .not_empty
            (
                &i18n.get
                (
                    "model_workspace.error.display_name.not_empty"
                )
            )
            .max_len
            (
                64,
                &i18n.get_with
                (
                    "model_workspace.error.display_name.max_len",
                    [("max_len", "64")].into()
                )
            )
            .validate();
        if display_name_validator.has_err()
        {
            return Err(display_name_validator.get_first_error());
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
    #[sea_orm(has_many = "super::project::Entity")]
    Project,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMember,

    #[sea_orm(has_many = "super::user_account_workspace::Entity")]
    UserAccountWorkspace,

    #[sea_orm(has_many = "super::organization_workspace::Entity")]
    OrganizationWorkspace,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::workspace_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMember.def()
    }
}

impl Related<super::user_account_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccountWorkspace.def()
    }
}

impl Related<super::organization_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationWorkspace.def()
    }
}
