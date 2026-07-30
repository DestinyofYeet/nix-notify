use crate::config::{
    ConfigError, Notification, NotificationKind, RawConfig, ValidatedEmailConfig,
    ValidatedNotification, ValidatedNotificationKind,
};

impl Notification {
    pub fn validate(self) -> Result<ValidatedNotification, ConfigError> {
        let Notification {
            kind,
            name,
            smtp_host,
            smtp_port,
            envelope_from,
            login_username,
            login_password,
        } = self;
        match kind {
            NotificationKind::Email => {
                let validate = |field: Option<String>, field_name: &str| {
                    RawConfig::validate_set_and_not_emtpy(field, &name, "Notification", &field_name)
                };

                let smtp_host = validate(smtp_host, "stmp_host")?;

                let smtp_port = match smtp_port {
                    Some(value) => value,
                    None => {
                        return Err(ConfigError::Validate(format!(
                            "Notification {name}: smtp_port is not set!"
                        )));
                    }
                };

                let envelope_from = validate(envelope_from, "envelope_from")?;
                let login_username = validate(login_username, "login_username")?;
                let login_password = RawConfig::validate_maybe_file_input(validate(
                    login_password,
                    "login_password",
                )?)?;

                let data = ValidatedEmailConfig {
                    smtp_host,
                    smtp_port,
                    envelope_from,
                    login_username,
                    login_password,
                };

                Ok(ValidatedNotification {
                    name,
                    kind: ValidatedNotificationKind::Email(data),
                })
            }
        }
    }
}
