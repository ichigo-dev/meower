//------------------------------------------------------------------------------
//! Linked.
//------------------------------------------------------------------------------

use super::account::Entity as AccountEntity;
use super::account_workspace::Relation as AccountWorkspaceRelation;
use super::group::Entity as GroupEntity;
use super::group_member::Relation as GroupMemberRelation;
use super::group_workspace::Relation as GroupWorkspaceRelation;
use super::workspace::Entity as WorkspaceEntity;

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Account -> Workspace.
//------------------------------------------------------------------------------
pub struct AccountToWorkspace;

impl Linked for AccountToWorkspace
{
    type FromEntity = AccountEntity;
    type ToEntity = WorkspaceEntity;

    fn link( &self ) -> Vec<RelationDef>
    {
        vec![AccountWorkspaceRelation::Workspace.def()]
    }
}


//------------------------------------------------------------------------------
/// Account -> Workspace (Group).
//------------------------------------------------------------------------------
pub struct AccountToGroupWorkspace;

impl Linked for AccountToGroupWorkspace
{
    type FromEntity = AccountEntity;
    type ToEntity = WorkspaceEntity;

    fn link( &self ) -> Vec<RelationDef>
    {
        vec!
        [
            GroupMemberRelation::Account.def().rev(),
            GroupMemberRelation::Group.def(),
            GroupWorkspaceRelation::Workspace.def(),
        ]
    }
}


//------------------------------------------------------------------------------
/// Account -> Group.
//------------------------------------------------------------------------------
pub struct AccountToGroup;

impl Linked for AccountToGroup
{
    type FromEntity = AccountEntity;
    type ToEntity = GroupEntity;

    fn link( &self ) -> Vec<RelationDef>
    {
        vec![GroupMemberRelation::Group.def()]
    }
}
