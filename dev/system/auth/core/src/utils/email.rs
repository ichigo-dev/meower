//------------------------------------------------------------------------------
//! Email utility. 
//------------------------------------------------------------------------------

use crate::config::Config;

use lettre::{ AsyncSmtpTransport, Tokio1Executor };
use lettre::transport::smtp::authentication::Credentials;


//------------------------------------------------------------------------------
/// Gets the mailer.
//------------------------------------------------------------------------------
pub(crate) fn get_mailer
(
    config: &Config,
) -> AsyncSmtpTransport<Tokio1Executor>
{
    let creds = Credentials::new
    (
        config.smtp_user.clone(),
        config.smtp_password.clone()
    );

    let host = config.smtp_host.clone();
    let port = config.smtp_port;
    if config.smtp_tls
    {
        AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
            .unwrap()
            .port(port)
            .credentials(creds)
            .build()
    }
    else
    {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host)
            .port(port)
            .credentials(creds)
            .build()
    }
}
