//------------------------------------------------------------------------------
//! UserAuth model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;
use argon2::{ Argon2, PasswordHash, PasswordVerifier };


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_auth")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_auth_id: i64,
    pub user_id: i64,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Verifies password.
    //--------------------------------------------------------------------------
    pub fn verify( &self, password: &str ) -> bool
    {
        let parsed_hash = PasswordHash::new(&password).unwrap();
        Argon2::default()
            .verify_password(self.password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModelBehavior for ActiveModel {}


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
