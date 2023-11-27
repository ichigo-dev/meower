//------------------------------------------------------------------------------
//! Account table model.
//------------------------------------------------------------------------------

use meower_utility::Validator;
use sea_orm::entity::prelude::*;
use sea_orm::{ ConnectionTrait, ActiveValue, ActiveModelTrait };
use sea_query::Condition;
use async_trait::async_trait;
use chrono::Utc;


//------------------------------------------------------------------------------
/// Account table model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_id: i32,
    pub user_id: i32,
    pub account_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation
{
    User,
}

impl RelationTrait for Relation
{
    fn def(&self) -> RelationDef
    {
        match self
        {
            Self::User =>
            {
                Entity::belongs_to(super::user::Entity)
                    .from(Column::UserId)
                    .to(super::user::Column::UserId)
                    .into()
            },
        }
    }
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
#[async_trait]
impl ActiveModelBehavior for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Creates a new user.
    //--------------------------------------------------------------------------
    fn new() -> Self
    {
        let now = Utc::now().naive_utc();
        Self
        {
            account_id: ActiveValue::NotSet,
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..ActiveModelTrait::default()
        }
    }

    //--------------------------------------------------------------------------
    /// Will be called before saving.
    //--------------------------------------------------------------------------
    async fn before_save<C>
    (
        mut self,
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let account_name = self.account_name.clone().unwrap();

        // Check if the account already exists
        if insert
        {
            let exists_user = Entity::find()
                .filter
                (
                    Condition::any()
                        .add(Column::AccountName.eq(account_name.clone()))
                )
                .one(hdb)
                .await
                .unwrap();
            if exists_user.is_some()
            {
                return Err(DbErr::Custom("The account already exists.".to_string()));
            }
        }

        // Validates account name.
        let mut account_name_validator = Validator::new(&account_name)
            .not_empty("Account name is required.")
            .min_len(3, "Account name is too short.")
            .max_len(255, "Account name is too long.")
            .regex(r"^[a-zA-Z0-9_]+$", "Account name must contain only letters, numbers, and underscores.")
            .validate();
        if account_name_validator.validate() == false
        {
            let errors = account_name_validator.errors();
            return Err(DbErr::Custom(errors[0].to_string()));
        }

        Ok(self)
    }
}
