use ptolemy::generated::record_publisher::Record;

#[derive(Debug)]
pub enum SinkMessage {
    Record(Record),
    Shutdown,
}

impl From<Record> for SinkMessage {
    fn from(value: Record) -> SinkMessage {
        SinkMessage::Record(value)
    }
}
