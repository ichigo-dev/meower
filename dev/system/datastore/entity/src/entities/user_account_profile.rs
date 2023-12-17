//------------------------------------------------------------------------------
//! UserAccountProfile model.
//------------------------------------------------------------------------------

use crate::Validate;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{ Serialize, Deserialize };


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize
)]
#[sea_orm(table_name = "user_account_profile")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_account_profile_id: i64,
    pub user_account_id: i64,
    pub name: Option<String>,
    pub biography: Option<String>,
    pub company: Option<String>,
    pub title: Option<String>,
    pub location: Option<String>,
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

impl Validate for ActiveModel {}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}
