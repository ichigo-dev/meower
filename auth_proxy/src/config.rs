//------------------------------------------------------------------------------
//! Configuration module.
//------------------------------------------------------------------------------

use std::env;


#[derive(Debug, Clone)]
pub struct Config
{
    port: u16,
    proxy_url: String,
    jwt_issue: String,
    jwt_subject: String,
    jwt_audience: Vec<String>,
    jwt_secret: String,
    jwt_expires: i64,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let port = env::var("AUTH_PROXY_PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .unwrap_or(8080);
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

        Self
        {
            port,
            proxy_url,
            jwt_issue,
            jwt_subject,
            jwt_audience,
            jwt_secret,
            jwt_expires,
        }
    }

    //--------------------------------------------------------------------------
    /// Returns the port number.
    //--------------------------------------------------------------------------
    pub(crate) fn port(&self) -> u16
    {
        self.port
    }

    //--------------------------------------------------------------------------
    /// Returns the proxy URL.
    //--------------------------------------------------------------------------
    pub(crate) fn proxy_url(&self) -> &str
    {
        &self.proxy_url
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT issue.
    //--------------------------------------------------------------------------
    pub(crate) fn jwt_issue(&self) -> &str
    {
        &self.jwt_issue
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT subject.
    //--------------------------------------------------------------------------
    pub(crate) fn jwt_subject(&self) -> &str
    {
        &self.jwt_subject
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT audience.
    //--------------------------------------------------------------------------
    pub(crate) fn jwt_audience(&self) -> &Vec<String>
    {
        &self.jwt_audience
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT secret.
    //--------------------------------------------------------------------------
    pub(crate) fn jwt_secret(&self) -> &str
    {
        &self.jwt_secret
    }

    //--------------------------------------------------------------------------
    /// Returns the JWT expiration.
    //--------------------------------------------------------------------------
    pub(crate) fn jwt_expires(&self) -> i64
    {
        self.jwt_expires
    }
}
