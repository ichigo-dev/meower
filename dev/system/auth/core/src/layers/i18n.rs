//------------------------------------------------------------------------------
//! Internationalization layer.
//------------------------------------------------------------------------------

use crate::state::AppState;

use axum::body::Body;
use axum::extract::State;
use axum::http::{ header, Request };
use axum::middleware::Next;
use axum::response::IntoResponse;


//------------------------------------------------------------------------------
/// Layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse
{
    let fallback_locale = state.config.fallback_locale;
    let language = req.headers()
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or(&fallback_locale);
    let user_language = accept_language::parse(language);
    rust_i18n::set_locale(&user_language[0]);
    next.run(req).await
}
