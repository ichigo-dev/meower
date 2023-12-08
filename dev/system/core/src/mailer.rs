//------------------------------------------------------------------------------
//! Mailer.
//------------------------------------------------------------------------------

use crate::Config;

use lettre::{ AsyncSmtpTransport, AsyncTransport, Tokio1Executor };
use lettre::Message;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;


//------------------------------------------------------------------------------
/// Mailer.
//------------------------------------------------------------------------------
pub struct Mailer
{
    inner: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer
{
    //--------------------------------------------------------------------------
    /// Creates a new Mailer.
    //--------------------------------------------------------------------------
    pub fn new( config: &Config ) -> Self
    {
        let creds = Credentials::new
        (
            config.get("smtp_user"),
            config.get("smtp_password"),
        );

        let host = &config.get("smtp_host");
        let inner = if config.get_as_bool("smtp_ssl")
        {
            AsyncSmtpTransport::<Tokio1Executor>::relay(host)
                .unwrap()
                .port(config.get_as_i64("smtp_port") as u16)
                .credentials(creds)
                .build()
        }
        else
        {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
                .port(config.get_as_i64("smtp_port") as u16)
                .credentials(creds)
                .build()
        };
        Mailer { inner }
    }

    //--------------------------------------------------------------------------
    /// Creates a new MessageBuilder.
    //--------------------------------------------------------------------------
    pub fn message() -> MessageBuilder
    {
        Message::builder()
    }

    //--------------------------------------------------------------------------
    /// Sends the mail.
    //--------------------------------------------------------------------------
    pub async fn send( &self, message: Message ) -> Result<(), String>
    {
        match self.inner.send(message).await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
