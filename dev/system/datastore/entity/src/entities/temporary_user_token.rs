//------------------------------------------------------------------------------
//! TemporaryUserToken model.
//------------------------------------------------------------------------------

use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "temporary_user_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_token_id: i64,
    pub temporary_user_id: i64,
    pub token: String,
    pub created_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Generates a token.
    //--------------------------------------------------------------------------
    pub fn generate_token() -> String
    {
        let mut rng = rand::thread_rng();
        let token: String = (0..6)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect();
        token
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
        // Generates and sets a token.
        let token = Model::generate_token();
        self.set(Column::Token, token.into());

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }

        Ok(self)
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::temporary_user::Entity",
        from = "Column::TemporaryUserId",
        to = "super::temporary_user::Column::TemporaryUserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TemporaryUser,
}

impl Related<super::temporary_user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TemporaryUser.def()
    }
}
