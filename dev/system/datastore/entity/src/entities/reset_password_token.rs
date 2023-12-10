//------------------------------------------------------------------------------
//! ResetPasswordToken model.
//------------------------------------------------------------------------------

use crate::GenerateToken;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "reset_password_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub reset_password_token_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    pub created_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user_code by token.
    //--------------------------------------------------------------------------
    pub async fn find_by_token<C>( hdb: &C, token: &str ) -> Option<Self>
    where
        C: ConnectionTrait,
    {
        let data = Entity::find()
            .filter(Column::Token.eq(token))
            .one(hdb)
            .await
            .unwrap_or(None);
        data
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
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Deletes the old tokens.
        if insert
        {
            let user_id = self.user_id.clone().unwrap();
            Entity::delete_many()
                .filter(Column::UserId.eq(user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());

            let token = self.generate_token();
            self.set(Column::Token, token.into());
        }

        Ok(self)
    }
}

impl GenerateToken for ActiveModel {}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}
