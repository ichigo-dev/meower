//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------

use std::env;


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct Config
{
    // Server config.
    pub(crate) port: u16,
    pub(crate) body_limit: usize,

    // Locale config.
    pub(crate) fallback_locale: String,

    // Database config.
    pub(crate) database_url: String,

    // Storage config.
    pub(crate) storage_url: String,
    pub(crate) storage_bucket: String,

    pub(crate) avatar_path: &'static str,
    pub(crate) default_avatar_path: &'static str,
    pub(crate) default_avatar_mime: &'static str,
    pub(crate) cover_path: &'static str,
    pub(crate) default_cover_path: &'static str,
    pub(crate) default_cover_mime: &'static str,

    pub(crate) group_avatar_path: &'static str,
    pub(crate) default_group_avatar_path: &'static str,
    pub(crate) default_group_avatar_mime: &'static str,
    pub(crate) group_cover_path: &'static str,
    pub(crate) default_group_cover_path: &'static str,
    pub(crate) default_group_cover_mime: &'static str,

    pub(crate) authorization_model_path: &'static str,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        // Server config.
        let port = env::var("PORT")
            .expect("PORT must be set")
            .parse::<u16>()
            .expect("PORT must be a number");
        let body_limit = env::var("BODY_LIMIT")
            .expect("BODY_LIMIT must be set")
            .parse::<usize>()
            .expect("BODY_LIMIT must be a number");

        // Locale config.
        let fallback_locale = env::var("FALLBACK_LOCALE")
            .expect("FALLBACK_LOCALE must be set");

        // Database config.
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Storage config.
        let storage_url = env::var("STORAGE_URL")
            .expect("STORAGE_URL must be set");
        let storage_bucket = env::var("STORAGE_BUCKET")
            .expect("STORAGE_BUCKET must be set");

        let avatar_path = "avatar";
        let default_avatar_path = "avatar/default.png";
        let default_avatar_mime = "image/png";
        let cover_path = "cover";
        let default_cover_path = "cover/default.jpg";
        let default_cover_mime = "image/jpg";

        let group_avatar_path = "group/avatar";
        let default_group_avatar_path = "group/avatar/default.png";
        let default_group_avatar_mime = "image/png";
        let group_cover_path = "group/cover";
        let default_group_cover_path = "group/cover/default.jpg";
        let default_group_cover_mime = "image/jpg";

        let authorization_model_path = "authorization/model.conf";

        Self
        {
            // Server config.
            port,
            body_limit,

            // Locale config.
            fallback_locale,

            // Database config.
            database_url,

            // Storage config.
            storage_url,
            storage_bucket,

            avatar_path,
            default_avatar_path,
            default_avatar_mime,
            cover_path,
            default_cover_path,
            default_cover_mime,

            group_avatar_path,
            default_group_avatar_path,
            default_group_avatar_mime,
            group_cover_path,
            default_group_cover_path,
            default_group_cover_mime,

            authorization_model_path,
        }
    }
}
