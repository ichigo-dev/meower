//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_install::common::install_master;
use meower_migration::Migrator;
use meower_core::{ Config, I18n };
use meower_entity::Validate;
use meower_entity::user::ActiveModel as ActiveUser;
use meower_entity::user_auth::ActiveModel as ActiveUserAuth;
use meower_entity::user_account::ActiveModel as ActiveUserAccount;
use meower_entity::user_account_profile::ActiveModel as ActiveUserAccountProfile;
use meower_entity::workspace::ActiveModel as ActiveWorkspace;
use meower_entity::user_account_workspace::ActiveModel as ActiveUserAccountWorkspace;
use meower_entity::workspace_member_authority::Entity as WorkspaceMemberAuthorityEntity;
use meower_entity::workspace_member_authority::AuthorityMap as WorkspaceMemberAuthorityMap;

use sea_orm_migration::MigratorTrait;
use sea_orm::{
    Database,
    ActiveValue,
    TransactionTrait,
};

#[tokio::main]
async fn main()
{
    let config = Config::new();
    let mut i18n = I18n::new();
    i18n.init("en", &config);

    let hdb = Database::connect(config.get("database.url"))
        .await
        .expect("Failed to setup the database");
    let tsx = hdb.begin().await.unwrap();

    // Refreshes the database.
    Migrator::refresh(&tsx).await.unwrap();

    // Installs master datas.
    if let Err(_) = install_master::install_master(&tsx, &i18n).await
    {
        tsx.rollback().await.unwrap();
        panic!("Failed to install master data");
    }

    // Installs test datas.
    for i in 1..=10
    {
        // Creates a user.
        let user = ActiveUser
        {
            email: ActiveValue::set(format!("user{}@example.com", i)),
            ..Default::default()
        };
        let user = match user.validate_and_insert(&tsx, &i18n).await
        {
            Ok(user) => user,
            Err(_) =>
            {
                tsx.rollback().await.unwrap();
                panic!("Failed to insert `user` test data");
            },
        };
        let user_id = user.user_id;

        // Creates a user auth.
        let user_auth = ActiveUserAuth
        {
            user_id: ActiveValue::Set(user_id),
            password: ActiveValue::set("password123!".to_string()),
            ..Default::default()
        };
        if user_auth.validate_and_insert(&tsx, &i18n).await.is_err()
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `user_auth` test data");
        }

        // Creates a user account.
        let user_account = ActiveUserAccount
        {
            user_id: ActiveValue::Set(user_id),
            user_account_name: ActiveValue::set(format!("user{}", i)),
            display_name: ActiveValue::set(format!("User {}", i)),
            ..Default::default()
        };
        let user_account = match user_account
            .validate_and_insert(&tsx, &i18n)
            .await
        {
            Ok(user_account) => user_account,
            Err(_) =>
            {
                tsx.rollback().await.unwrap();
                panic!("Failed to insert `user_account` test data");
            },
        };
        let user_account_id = user_account.user_account_id;

        // Creates a user account profile.
        let user_account_profile = ActiveUserAccountProfile
        {
            user_account_id: ActiveValue::Set(user_account_id),
            name: ActiveValue::set(Some(format!("User {}", i))),
            biography: ActiveValue::set(Some("Hello, world!".to_string())),
            company: ActiveValue::set(Some("Example Inc.".to_string())),
            title: ActiveValue::set(Some("Software Engineer".to_string())),
            location: ActiveValue::set(Some("Tokyo, Japan".to_string())),
            ..Default::default()
        };
        if user_account_profile.validate_and_insert(&tsx, &i18n).await.is_err()
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `user_account_profile` test data");
        }

        // Creates a workspace.
        let user_account_name = user_account.user_account_name;
        let display_name = format!
        (
            "{}'s workspace",
            user_account_name.clone()
        );
        let workspace = ActiveWorkspace
        {
            workspace_name: ActiveValue::set(user_account_name.clone()),
            display_name: ActiveValue::set(display_name),
            ..Default::default()
        };
        let workspace = match workspace.validate_and_insert(&tsx, &i18n).await
        {
            Ok(workspace) => workspace,
            Err(_) =>
            {
                tsx.rollback().await.unwrap();
                panic!("Failed to insert `workspace` test data");
            },
        };

        // Creates a user_account_workspace.
        let user_account_workspace = ActiveUserAccountWorkspace
        {
            user_account_id: ActiveValue::Set(user_account_id),
            workspace_id: ActiveValue::Set(workspace.workspace_id),
            ..Default::default()
        };
        if let Err(_) = user_account_workspace
            .validate_and_insert(&tsx, &i18n)
            .await
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `user_account_workspace` test data");
        }

        // Creates a workspace member.
        let authority = WorkspaceMemberAuthorityEntity::find_by_authority
        (
            &WorkspaceMemberAuthorityMap::Admin
        )
            .one(&tsx)
            .await
            .unwrap()
            .unwrap();
        let authority_id = authority.workspace_member_authority_id;
        let workspace_member = meower_entity::workspace_member::ActiveModel
        {
            workspace_id: ActiveValue::Set(workspace.workspace_id),
            user_account_id: ActiveValue::Set(user_account_id),
            workspace_member_authority_id: ActiveValue::Set(authority_id),
            ..Default::default()
        };
        if let Err(_) = workspace_member.validate_and_insert(&tsx, &i18n).await
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `workspace_member` test data");
        }
    }

    tsx.commit().await.unwrap();
    println!("=== Test data installed ===");
}
