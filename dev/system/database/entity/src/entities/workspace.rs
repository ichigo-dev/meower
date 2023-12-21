//------------------------------------------------------------------------------
//! Workspace model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;

use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Workspace: The workspace name already exists.")]
    AlreadyExistsWorkspaceName,

    #[error("Workspace: {0}")]
    Validation(#[from] ValidationError),

    #[error("Workspace: Database error.")]
    DbError(#[from] DbErr),
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
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
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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
impl ValidateExt for ActiveModel
{
    type Error = Error;

    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>( &self, hdb: &C ) -> Result<(), Self::Error>
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
                return Err(Error::AlreadyExistsWorkspaceName);
            }
        }

        // Validates fields.
        Validator::new()
            .required()
            .min_length(3)
            .max_length(32)
            .regex(r"^[a-zA-Z0-9_]+$")
            .validate(&workspace_name)?;

        let display_name = self.display_name.clone().unwrap();
        Validator::new()
            .required()
            .max_length(64)
            .validate(&display_name)?;

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
