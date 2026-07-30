use crate::notifications::SendNotification;

impl SendNotification {
    pub fn new(name: String, recipient: String, title: String, text: String) -> Self {
        Self {
            name,
            recipient,
            title,
            text,
        }
    }
}
