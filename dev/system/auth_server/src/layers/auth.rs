//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth };
use meower_entity::user::Entity as UserEntity;
use meower_entity::user_jwt_subject::Entity as UserJwtSubjectEntity;
use meower_entity::user_account::{ self, Entity as UserAccountEntity };

use axum::response::{ IntoResponse, Redirect };
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;
use sea_orm::prelude::*;
use sea_orm::QueryOrder;


//------------------------------------------------------------------------------
/// Authentication layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    cookie: CookieJar,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let hdb = state.hdb();
    let config = state.config();

    // Initializes the authentication.
    let auth = Auth::init_from_cookie(&cookie, config);
    let path= req.uri().path();

    // Skips the authentication for assets and create account page.
    let is_assets = path.starts_with("/_assets");
    if is_assets
    {
        return Ok(next.run(req).await);
    }

    let is_logined = auth.is_logined().await;
    let is_auth_page = path.starts_with("/auth");
    if !is_logined && !is_auth_page
    {
        return Err(Redirect::to("/auth/login"));
    }

    if is_logined
    {
        let is_create_account_page = path.starts_with("/auth/create_account");
        if is_auth_page && !is_create_account_page
        {
            return Err(Redirect::to("/"));
        }

        let claims = match auth.claims()
        {
            Some(claims) => claims,
            None => return Err(Redirect::to("/auth/login")),
        };

        // Finds the subject of the JWT.
        let user_jwt_subject =
            match UserJwtSubjectEntity::find_by_subject(&claims.sub)
                .one(hdb)
                .await
                .unwrap()
        {
            Some(user_jwt_subject) => user_jwt_subject,
            None => return Err(Redirect::to("/auth/login")),
        };

        // Finds the logined user.
        let user_id = user_jwt_subject.user_id;
        let user = match UserEntity::find_by_id(user_id)
            .one(hdb)
            .await
            .unwrap()
        {
            Some(user) => user,
            None => return Err(Redirect::to("/auth/login")),
        };

        // Finds a last logined user account.
        match UserAccountEntity::find()
            .filter(user_account::Column::UserId.eq(user.user_id))
            .order_by_desc(user_account::Column::LastLoginedAt)
            .one(hdb)
            .await
            .unwrap()
        {
            Some(user_account) =>
            {
            },
            None =>
            {
                if is_create_account_page
                {
                    return Ok(next.run(req).await);
                }

                let redirect_path = format!
                (
                    "/auth/create_account/{}",
                    &claims.sub,
                );
                return Err(Redirect::to(&redirect_path));
            },
        };
    }

    Ok(next.run(req).await)
}
