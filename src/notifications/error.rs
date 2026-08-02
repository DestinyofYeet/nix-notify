use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Failed to send {} notification: {}", .kind, .msg)]
    Send { kind: String, msg: String },
}
