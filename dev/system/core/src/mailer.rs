//------------------------------------------------------------------------------
//! Mailer.
//------------------------------------------------------------------------------

use crate::{ Config, I18n };

use std::fs;
use std::collections::HashMap;

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
            config.get("smtp.user"),
            config.get("smtp.password"),
        );

        let host = &config.get("smtp.host");
        let inner = if config.get_as_bool("smtp.ssl")
        {
            AsyncSmtpTransport::<Tokio1Executor>::relay(host)
                .unwrap()
                .port(config.get_as_isize("smtp.port") as u16)
                .credentials(creds)
                .build()
        }
        else
        {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
                .port(config.get_as_isize("smtp.port") as u16)
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
    /// Gets template mail.
    //--------------------------------------------------------------------------
    pub fn get_template
    (
        template: &str,
        config: &Config,
        i18n: &I18n,
    ) -> String
    {
        let email_path = config.get("email.path");
        let template_path = format!
        (
            "{}/{}/{}",
            email_path,
            i18n.locale(),
            template,
        );
        println!("{}", template_path);
        let template = fs::read_to_string(template_path)
            .unwrap_or("".to_string());
        template
    }

    //--------------------------------------------------------------------------
    /// Gets template mail.
    //--------------------------------------------------------------------------
    pub fn get_template_with
    (
        template: &str,
        config: &Config,
        i18n: &I18n,
        replace: HashMap<&str, &str>,
    ) -> String
    {
        let mut template = Self::get_template(template, config, i18n);
        for (key, value) in replace
        {
            template = template.replace(&format!("%{}%", key), &value);
        }
        template
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
