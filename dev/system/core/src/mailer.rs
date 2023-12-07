//------------------------------------------------------------------------------
//! Mailer.
//------------------------------------------------------------------------------

use crate::Config;

use lettre::{ Message, SmtpTransport, Transport };
use lettre::transport::smtp::authentication::Credentials;


//------------------------------------------------------------------------------
/// Mailer.
//------------------------------------------------------------------------------
pub struct Mailer
{
    message: Message,
}

impl Mailer
{
    //--------------------------------------------------------------------------
    /// Creates a new Mailer.
    //--------------------------------------------------------------------------
    pub fn new() -> MailBuilder
    {
        MailBuilder::new()
    }

    //--------------------------------------------------------------------------
    /// Generates credentials.
    //--------------------------------------------------------------------------
    pub fn get_credentials( &self, config: &Config ) -> Credentials
    {
        Credentials::new
        (
            config.get("smtp_user"),
            config.get("smtp_password"),
        )
    }

    //--------------------------------------------------------------------------
    /// Gets mailer.
    //--------------------------------------------------------------------------
    pub fn get_mailer( &self, config: &Config ) -> SmtpTransport
    {
        let creds = self.get_credentials(config);
        if config.get_as_bool("smtp_ssl")
        {
            SmtpTransport::relay(&config.get("smtp_host"))
                .unwrap()
                .port(config.get_as_i64("smtp_port") as u16)
                .credentials(creds)
                .build()
        }
        else
        {
            SmtpTransport::builder_dangerous(&config.get("smtp_host"))
                .port(config.get_as_i64("smtp_port") as u16)
                .credentials(creds)
                .build()
        }
    }

    //--------------------------------------------------------------------------
    /// Sends the mail.
    //--------------------------------------------------------------------------
    pub fn send( &self, config: &Config ) -> Result<(), String>
    {
        let mailer = self.get_mailer(&config);
        match mailer.send(&self.message)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}


//------------------------------------------------------------------------------
/// MailBuilder.
//------------------------------------------------------------------------------
pub struct MailBuilder
{
    from: String,
    to: String,
    subject: String,
    body: String,
}

impl MailBuilder
{
    //--------------------------------------------------------------------------
    /// Creates a new MailBuilder.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self
        {
            from: String::new(),
            to: String::new(),
            subject: String::new(),
            body: String::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Sets the from address.
    //--------------------------------------------------------------------------
    pub fn from( mut self, from: &str ) -> Self
    {
        self.from = from.to_string();
        self
    }

    //--------------------------------------------------------------------------
    /// Sets the to address.
    //--------------------------------------------------------------------------
    pub fn to( mut self, to: &str ) -> Self
    {
        self.to = to.to_string();
        self
    }

    //--------------------------------------------------------------------------
    /// Sets the subject.
    //--------------------------------------------------------------------------
    pub fn subject( mut self, subject: &str ) -> Self
    {
        self.subject = subject.to_string();
        self
    }

    //--------------------------------------------------------------------------
    /// Sets the body.
    //--------------------------------------------------------------------------
    pub fn body( mut self, body: &str ) -> Self
    {
        self.body = body.to_string();
        self
    }

    //--------------------------------------------------------------------------
    /// Builds the Mailer.
    //--------------------------------------------------------------------------
    pub fn build( self ) -> Mailer
    {
        let message = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(self.subject)
            .body(self.body)
            .unwrap();

        Mailer { message }
    }
}
