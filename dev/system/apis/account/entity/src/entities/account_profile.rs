//------------------------------------------------------------------------------
//! AccountProfile model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use async_graphql::{ Enum, SimpleObject };
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Gender.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "i8", db_type = "TinyInteger")]
pub enum Gender
{
    Male = 1,
    Female = 2,
    Other = 99,
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "account_profile")]
#[graphql(concrete(name = "AccountProfile", params()))]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_profile_id: i64,
    pub account_id: i64,
    pub name: String,
    pub affiliation: String,
    pub bio: String,
    pub email: String,
    pub birthdate: DateTime,
    pub gender: Option<Gender>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
    async fn validate<C>( &self, _hdb: &C ) -> Result<(), Self::Error>
    where
        C: ConnectionTrait,
    {
        let name = self.name.clone().take().unwrap_or("".to_string());
        let affiliation = self.affiliation.clone().take().unwrap_or("".to_string());
        let bio = self.bio.clone().take().unwrap_or("".to_string());
        let email = self.email.clone().take().unwrap_or("".to_string());
        let birthdate = self
            .birthdate
            .clone()
            .take()
            .unwrap_or(DateTime::from_timestamp_millis(0).unwrap());

        // Validates fields.
        if let Err(e) = Validator::new()
            .max_length(32)
            .validate(&name)
        {
            return Err(Error::Validation { column: Column::Name, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(32)
            .validate(&affiliation)
        {
            return Err(Error::Validation { column: Column::Affiliation, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(1024)
            .validate(&bio)
        {
            return Err(Error::Validation { column: Column::Bio, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(255)
            .is_email()
            .validate(&email)
        {
            return Err(Error::Validation { column: Column::Email, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(255)
            .is_email()
            .custom(Box::new(|value|
            {
                let now = Utc::now().naive_utc().timestamp_millis().to_string();
                if value > now.as_str()
                {
                    return Err(ValidationError::Custom
                    (
                        t!("entities.account_profile.birthdate.error.future")
                    ));
                }
                Ok(())
            }))
            .validate(&birthdate.timestamp_millis().to_string())
        {
            return Err(Error::Validation { column: Column::Email, error: e });
        }

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Column.
//------------------------------------------------------------------------------
impl Column
{
    //--------------------------------------------------------------------------
    /// Gets the column name.
    //--------------------------------------------------------------------------
    pub fn get_name( &self ) -> String
    {
        match self
        {
            Self::AccountProfileId => t!("entities.account_profile.account_profile_id.name"),
            Self::AccountId => t!("entities.account_profile.account_id.name"),
            Self::Name => t!("entities.account_profile.name.name"),
            Self::Affiliation => t!("entities.account_profile.affiliation.name"),
            Self::Bio => t!("entities.account_profile.bio.name"),
            Self::Email => t!("entities.account_profile.email.name"),
            Self::Birthdate => t!("entities.account_profile.birthdate.name"),
            Self::Gender => t!("entities.account_profile.gender.name"),
            Self::CreatedAt => t!("entities.account_profile.created_at.name"),
            Self::UpdatedAt => t!("entities.account_profile.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("AccountProfile: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("AccountProfile: Database error.")]
    DbError(#[from] DbErr),
}

impl Error
{
    //--------------------------------------------------------------------------
    /// Gets the error columns.
    //--------------------------------------------------------------------------
    pub fn get_column( &self ) -> Option<Column>
    {
        match self
        {
            Self::Validation { column, .. } => Some(*column),
            Self::DbError(_) => None,
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the error message.
    //--------------------------------------------------------------------------
    pub fn get_message( &self ) -> String
    {
        match self
        {
            Self::Validation { column, error } =>
            {
                error.get_error_message(&column.get_name())
            },
            Self::DbError(_) => t!("common.error.db"),
        }
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::AccountId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Account,

    #[sea_orm(has_many = "super::group_member::Entity")]
    GroupMember,
}

impl Related<super::account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Account.def()
    }
}

impl Related<super::group_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupMember.def()
    }
}
