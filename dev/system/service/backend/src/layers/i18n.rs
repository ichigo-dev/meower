//------------------------------------------------------------------------------
//! Module for user i18n.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n };

use axum::response::IntoResponse;
use axum::body::Body;
use axum::http::header;
use axum::http::Request;
use axum::middleware::Next;
use axum::extract::State;


//------------------------------------------------------------------------------
/// Authentication layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer
(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse
{
    let config = &state.config;

    // Gets the user language.
    let fallback_locale = config.get("locale.default");
    let language = req.headers()
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or(&fallback_locale);
    let user_language = accept_language::parse(language);

    // Initializes the i18n.
    let mut i18n = I18n::new();
    for language in user_language
    {
        if i18n.init(&language, config) == true
        {
            break;
        }
    }
    req.extensions_mut().insert(i18n);
    next.run(req).await
}
