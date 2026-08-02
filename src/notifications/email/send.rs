use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        client::{Tls, TlsParameters},
    },
};

use crate::notifications::{email::SendEmail, error::NotificationError};

impl<'a> SendEmail<'a> {
    pub fn send(self) -> Result<(), NotificationError> {
        let email = Message::builder()
            .from(Mailbox::new(
                None,
                self.email_config
                    .envelope_from
                    .parse()
                    .map_err(|e| NotificationError::Send {
                        kind: "E-Mail".to_string(),
                        msg: format!("Failed to construct sending mailbox: {e}"),
                    })?,
            ))
            .to(Mailbox::new(
                None,
                self.recipient
                    .parse()
                    .map_err(|e| NotificationError::Send {
                        kind: "E-Mail".to_string(),
                        msg: format!("Failed to construct recipient mailbox: {e}"),
                    })?,
            ))
            .subject(self.title)
            .header(ContentType::TEXT_PLAIN)
            .body(self.content.to_string())
            .map_err(|e| NotificationError::Send {
                kind: "E-Mail".to_string(),
                msg: format!("Failed to construct email: {e}"),
            })?;

        let credentials = Credentials::new(
            self.email_config.login_username.clone(),
            self.email_config.login_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.email_config.smtp_host)
            .map_err(|e| NotificationError::Send {
                kind: "E-Mail".to_string(),
                msg: format!("Failed to construct relay config: {e}"),
            })?
            .credentials(credentials)
            .authentication(vec![Mechanism::Plain])
            .port(self.email_config.smtp_port.get())
            .tls(Tls::Wrapper(
                TlsParameters::new(self.email_config.smtp_host.clone()).map_err(|e| {
                    NotificationError::Send {
                        kind: "E-Mail".to_string(),
                        msg: format!("Failed to construct tls config: {e}"),
                    }
                })?,
            ))
            .build();

        mailer.send(&email).map_err(|e| NotificationError::Send {
            kind: "E-Mail".to_string(),
            msg: format!("Failed to send email: {e}"),
        })?;

        Ok(())
    }
}
