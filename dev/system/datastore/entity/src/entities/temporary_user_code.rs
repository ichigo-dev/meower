//------------------------------------------------------------------------------
//! TemporaryUserCode model.
//------------------------------------------------------------------------------

use meower_core::{ Config, I18n };
use crate::GenerateToken;

use async_trait::async_trait;
use chrono::{ Utc, Duration };
use rand::Rng;
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "temporary_user_code")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_code_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    pub temporary_user_id: i64,
    pub code: String,
    pub created_at: DateTime,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user_code by token.
    //--------------------------------------------------------------------------
    pub fn find_by_token( token: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Token.eq(token))
    }
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Generates a code.
    //--------------------------------------------------------------------------
    pub fn generate_code() -> String
    {
        let mut rng = rand::thread_rng();
        let code: String = (0..6)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect();
        code
    }

    //--------------------------------------------------------------------------
    /// Verifies a code.
    //--------------------------------------------------------------------------
    pub fn verify_code
    (
        &self,
        code: &str,
        config: &Config,
        i18n: &I18n,
    ) -> Result<(), String>
    {
        let expire = config.get_as_isize("verify_code.expire_sec");
        let expire_date = self.created_at + Duration::seconds(expire as i64);
        if Utc::now().naive_utc() > expire_date
        {
            return Err
            (
                i18n.get("model_temporary_user_code.error.code.expired")
            );
        }
        if self.code != code
        {
            return Err
            (
                i18n.get("model_temporary_user_code.error.code.not_match")
            );
        }
        Ok(())
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
        // Deletes the old codes.
        if insert
        {
            let temporary_user_id = self.temporary_user_id.clone().unwrap();
            Entity::delete_many()
                .filter(Column::TemporaryUserId.eq(temporary_user_id))
                .exec(hdb)
                .await?;
        }

        // Generates and sets a code.
        let code = Model::generate_code();
        self.set(Column::Code, code.into());

        // Sets the default values.
        if insert
        {
            let now = Utc::now().naive_utc();
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
