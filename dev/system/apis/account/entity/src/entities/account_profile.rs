//------------------------------------------------------------------------------
//! AccountProfile model.
//------------------------------------------------------------------------------

use super::account::Column as AccountColumn;
use super::account::Entity as AccountEntity;
use super::account::Model as AccountModel;
use super::account_profile_avatar::Model as AccountProfileAvatar;
use super::account_profile_cover::Model as AccountProfileCover;
use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use std::sync::Arc;

use async_graphql::{ Context, Enum, Object };
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::{ DatabaseTransaction, DeleteResult };
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Gender.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Gender
{
    Male = 1,
    Female = 2,
    Other = 99,
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "account_profile")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_profile_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    pub account_id: i64,
    pub name: String,
    pub affiliation: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub telno: Option<String>,
    pub birthdate: Option<DateTime>,
    pub gender: Option<Gender>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[Object(name = "AccountProfile")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the token.
    //--------------------------------------------------------------------------
    pub async fn token( &self ) -> String
    {
        self.token.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the name.
    //--------------------------------------------------------------------------
    pub async fn name( &self ) -> String
    {
        self.name.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the affiliation.
    //--------------------------------------------------------------------------
    pub async fn affiliation( &self ) -> Option<String>
    {
        self.affiliation.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the location.
    //--------------------------------------------------------------------------
    pub async fn location( &self ) -> Option<String>
    {
        self.location.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the bio.
    //--------------------------------------------------------------------------
    pub async fn bio( &self ) -> Option<String>
    {
        self.bio.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the email.
    //--------------------------------------------------------------------------
    pub async fn email( &self ) -> Option<String>
    {
        self.email.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the telno.
    //--------------------------------------------------------------------------
    pub async fn telno( &self ) -> Option<String>
    {
        self.telno.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the birthdate.
    //--------------------------------------------------------------------------
    pub async fn birthdate( &self ) -> Option<DateTime>
    {
        self.birthdate
    }

    //--------------------------------------------------------------------------
    /// Gets the gender.
    //--------------------------------------------------------------------------
    pub async fn gender( &self ) -> Option<Gender>
    {
        self.gender
    }

    //--------------------------------------------------------------------------
    /// Gets the create date.
    //--------------------------------------------------------------------------
    pub async fn created_at( &self ) -> DateTime
    {
        self.created_at
    }

    //--------------------------------------------------------------------------
    /// Gets the update date.
    //--------------------------------------------------------------------------
    pub async fn updated_at( &self ) -> DateTime
    {
        self.updated_at
    }

    //--------------------------------------------------------------------------
    /// Gets the account.
    //--------------------------------------------------------------------------
    pub async fn account( &self, ctx: &Context<'_> ) -> Option<AccountModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(AccountEntity).one(tsx).await.unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the avatar.
    //--------------------------------------------------------------------------
    pub async fn avatar
    (
        &self,
        ctx: &Context<'_>,
    ) -> Option<AccountProfileAvatar>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(super::account_profile_avatar::Entity)
            .one(tsx)
            .await
            .unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the cover.
    //--------------------------------------------------------------------------
    pub async fn cover
    (
        &self,
        ctx: &Context<'_>,
    ) -> Option<AccountProfileCover>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(super::account_profile_cover::Entity)
            .one(tsx)
            .await
            .unwrap()
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
            let token = meower_utility::get_random_str(128);
            self.set(Column::Token, token.into());

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
        let name = self.name
            .clone()
            .take()
            .unwrap_or("".to_string());
        let affiliation = self.affiliation
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_string());
        let location = self.location
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_string());
        let bio = self.bio
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_string());
        let email = self.email
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_string());
        let telno = self.telno
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or("".to_string());
        let birthdate = self
            .birthdate
            .clone()
            .take()
            .unwrap_or(None)
            .unwrap_or(DateTime::from_timestamp_millis(0).unwrap());

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .max_length(32)
            .validate(&name)
        {
            return Err(Error::Validation { column: Column::Name, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(32)
            .validate(&affiliation)
        {
            return Err
            (
                Error::Validation { column: Column::Affiliation, error: e }
            );
        }

        if let Err(e) = Validator::new()
            .max_length(32)
            .validate(&location)
        {
            return Err
            (
                Error::Validation { column: Column::Location, error: e }
            );
        }

        if let Err(e) = Validator::new()
            .max_length(1024)
            .validate(&bio)
        {
            return Err(Error::Validation { column: Column::Bio, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(255)
            .validate(&email)
        {
            return Err(Error::Validation { column: Column::Email, error: e });
        }

        if let Err(e) = Validator::new()
            .max_length(32)
            .validate(&telno)
        {
            return Err
            (
                Error::Validation { column: Column::Telno, error: e }
            );
        }

        if let Err(e) = Validator::new()
            .max_length(255)
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
            return Err
            (
                Error::Validation { column: Column::Birthdate, error: e }
            );
        }

        Ok(())
    }

    //--------------------------------------------------------------------------
    /// Validates the data before delete.
    //--------------------------------------------------------------------------
    async fn validate_and_delete<C>
    (
        self,
        hdb: &C,
    ) -> Result<DeleteResult, Self::Error>
    where
        C: ConnectionTrait,
    {
        let account_profile_id = self.account_profile_id
            .clone()
            .take()
            .unwrap_or(0);
        if let Some(_) = AccountEntity::find()
            .filter(AccountColumn::DefaultAccountProfileId.eq(account_profile_id))
            .one(hdb)
            .await?
        {
            return Err(Error::DefaultAccountProfile);
        };

        match self.delete(hdb).await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::DbError(e)),
        }
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
            Self::Token => t!("entities.account_profile.token.name"),
            Self::AccountId => t!("entities.account_profile.account_id.name"),
            Self::Name => t!("entities.account_profile.name.name"),
            Self::Affiliation => t!("entities.account_profile.affiliation.name"),
            Self::Location => t!("entities.account_profile.location.name"),
            Self::Bio => t!("entities.account_profile.bio.name"),
            Self::Email => t!("entities.account_profile.email.name"),
            Self::Telno => t!("entities.account_profile.telno.name"),
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
    #[error("AccountProfile: This account profile is default.")]
    DefaultAccountProfile,

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
            Self::DefaultAccountProfile => None,
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
            Self::DefaultAccountProfile =>
            {
                t!("entities.account_profile.error.default_account_profile")
            },
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

    #[sea_orm(has_one = "super::account_profile_avatar::Entity")]
    AccountProfileAvatar,

    #[sea_orm(has_one = "super::account_profile_cover::Entity")]
    AccountProfileCover,

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

impl Related<super::account_profile_avatar::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountProfileAvatar.def()
    }
}

impl Related<super::account_profile_cover::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountProfileCover.def()
    }
}

impl Related<super::group_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupMember.def()
    }
}
