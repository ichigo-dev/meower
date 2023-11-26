//------------------------------------------------------------------------------
//! Configuration module.
//------------------------------------------------------------------------------

use std::env;


//------------------------------------------------------------------------------
/// Configuration.
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Config
{
    // Application configuration.
    debug_mode: bool,
    database_url: String,
    proxy_url: String,

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
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub fn init() -> Self
    {
        let debug_mode = env::var("DEBUG_MODE")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let database_url = env::var("PROXY_URL")
            .unwrap_or("".to_string());
        let proxy_url = env::var("PROXY_URL")
            .unwrap_or("http://frontend:9000".to_string());

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
            proxy_url,
            jwt_issue,
            jwt_subject,
            jwt_audience,
            jwt_secret,
            jwt_expires,
            argon2_phc_salt,
        }
    }

    //--------------------------------------------------------------------------
    /// Returns the debug mode.
    //--------------------------------------------------------------------------
    pub fn debug_mode(&self) -> bool
    {
        self.debug_mode
    }

    //--------------------------------------------------------------------------
    /// Returns the database URL.
    //--------------------------------------------------------------------------
    pub fn database_url(&self) -> &str
    {
        &self.database_url
    }

    //--------------------------------------------------------------------------
    /// Returns the proxy URL.
    //--------------------------------------------------------------------------
    pub fn proxy_url(&self) -> &str
    {
        &self.proxy_url
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT issue.
    //--------------------------------------------------------------------------
    pub fn jwt_issue(&self) -> &str
    {
        &self.jwt_issue
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT subject.
    //--------------------------------------------------------------------------
    pub fn jwt_subject(&self) -> &str
    {
        &self.jwt_subject
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT audience.
    //--------------------------------------------------------------------------
    pub fn jwt_audience(&self) -> &Vec<String>
    {
        &self.jwt_audience
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT secret.
    //--------------------------------------------------------------------------
    pub fn jwt_secret(&self) -> &str
    {
        &self.jwt_secret
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT expiration.
    //--------------------------------------------------------------------------
    pub fn jwt_expires(&self) -> i64
    {
        self.jwt_expires
    }

    //--------------------------------------------------------------------------
    /// Returns the Argon2 PHC salt.
    //--------------------------------------------------------------------------
    pub fn argon2_phc_salt(&self) -> &str
    {
        &self.argon2_phc_salt
    }
}
