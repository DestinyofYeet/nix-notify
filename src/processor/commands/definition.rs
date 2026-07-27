use crate::feed::feeditem::FeedItem;

#[derive(Debug, Clone)]
pub enum ProcessorCommand {
    Process(Vec<FeedItem>),
}
