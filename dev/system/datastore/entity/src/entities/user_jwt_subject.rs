//------------------------------------------------------------------------------
//! UserJwtSubject model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;
use crate::GenerateToken;

use async_trait::async_trait;
use chrono::Utc;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_jwt_subject")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_jwt_subject_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub subject: String,
    pub created_at: DateTime,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user_jwt_subject by subject.
    //--------------------------------------------------------------------------
    pub fn find_by_subject( subject: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Subject.eq(subject))
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
        // Deletes the old subjects.
        if insert
        {
            let user_id = self.user_id.clone().unwrap();
            Entity::delete_many()
                .filter(Column::UserId.eq(user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        if insert
        {
            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());

            let token = self.generate_token();
            self.set(Column::Subject, token.into());
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
