//------------------------------------------------------------------------------
//! Configuration module.
//------------------------------------------------------------------------------

use std::env;
use std::cell::OnceCell;

const GLOBAL_CONFIG: OnceCell<Config> = OnceCell::new();


//------------------------------------------------------------------------------
/// Configuration.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Config
{
    // Application configuration.
    debug_mode: bool,
    database_url: String,
    application_url: String,

    // Locale configuration.
    locale_path: String,
    fallback_locale: String,

    // JWT configuration.
    jwt_issue: String,
    jwt_subject: String,
    jwt_audience: Vec<String>,
    jwt_secret: String,
    jwt_expires: i64,

    // Argon2 configuration.
    argon2_phc_salt: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Gets the global configuration.
    //--------------------------------------------------------------------------
    pub fn get() -> Self
    {
        GLOBAL_CONFIG.get_or_init(|| Self::init()).clone()
    }

    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub fn init() -> Self
    {
        let debug_mode = env::var("DEBUG_MODE")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or("".to_string());
        let application_url = env::var("APPLICATION_URL")
            .unwrap_or("http://frontend:9000".to_string());

        // Locale configuration.
        let locale_path = env::var("LOCALE_PATH")
            .unwrap_or("./locale".to_string());
        let fallback_locale = env::var("FALLBACK_LOCALE")
            .unwrap_or("en".to_string());

        // JWT configuration.
        let jwt_issue = env::var("JWT_ISSUE")
            .unwrap_or("meower".to_string());
        let jwt_subject = env::var("JWT_SUBJECT")
            .unwrap_or("meower".to_string());
        let jwt_audience = env::var("JWT_AUDIENCE")
            .unwrap_or("meower".to_string())
            .split(",")
            .map(|v| v.to_string())
            .collect();
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or("secret".to_string());
        let jwt_expires = env::var("JWT_EXPIRES")
            .unwrap_or("3600".to_string())
            .parse()
            .unwrap_or(3600);

        // Argon2 configuration.
        let argon2_phc_salt = env::var("ARGON2_PHC_SALT")
            .unwrap_or("salt".to_string());

        Self
        {
            debug_mode,
            database_url,
            application_url,
            locale_path,
            fallback_locale,
            jwt_issue,
            jwt_subject,
            jwt_audience,
            jwt_secret,
            jwt_expires,
            argon2_phc_salt,
        }
    }

    //--------------------------------------------------------------------------
    /// Returns configuration value.
    //--------------------------------------------------------------------------

    // Application configuration.
    pub fn debug_mode(&self) -> bool
    {
        self.debug_mode
    }

    pub fn database_url(&self) -> &str
    {
        &self.database_url
    }

    pub fn application_url(&self) -> &str
    {
        &self.application_url
    }

    // Locale configuration.
    pub fn locale_path(&self) -> &str
    {
        &self.locale_path
    }

    pub fn fallback_locale(&self) -> &str
    {
        &self.fallback_locale
    }

    // JWT configuration.
    pub fn jwt_issue(&self) -> &str
    {
        &self.jwt_issue
    }

    pub fn jwt_subject(&self) -> &str
    {
        &self.jwt_subject
    }

    pub fn jwt_audience(&self) -> &Vec<String>
    {
        &self.jwt_audience
    }

    pub fn jwt_secret(&self) -> &str
    {
        &self.jwt_secret
    }

    pub fn jwt_expires(&self) -> i64
    {
        self.jwt_expires
    }

    // Argon2 configuration.
    pub fn argon2_phc_salt(&self) -> &str
    {
        &self.argon2_phc_salt
    }
}
