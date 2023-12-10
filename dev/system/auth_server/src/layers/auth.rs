//------------------------------------------------------------------------------
//! Module for user authentication.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth };
use meower_entity::user::Entity as UserEntity;
use meower_entity::user_jwt_subject::Entity as UserJwtSubjectEntity;

use axum::response::{ IntoResponse, Redirect };
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;
use sea_orm::prelude::*;


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
    let config = state.config();

    // Initializes the authentication.
    let auth = Auth::init_from_cookie(&cookie, config);

    let is_logined = auth.is_logined().await;
    let is_auth_page = req.uri().path().starts_with("/auth");
    if !is_logined && !is_auth_page
    {
        return Err(Redirect::to("/auth/login"));
    }

    let hdb = state.hdb();
    if is_logined
    {
        if is_auth_page
        {
            return Err(Redirect::to("/"));
        }

        // Finds the subject of the JWT.
        let sub = match auth.claims()
        {
            Some(claims) => &claims.sub,
            None => return Err(Redirect::to("/auth/login")),
        };
        let user_jwt_subject = match UserJwtSubjectEntity::find_by_subject(sub)
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
        {
            Ok(user) => user,
            Err(_) => return Err(Redirect::to("/auth/login")),
        };
    }

    Ok(next.run(req).await)
}
