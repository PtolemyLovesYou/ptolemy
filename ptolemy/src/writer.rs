use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Message<T> {
    Write(T),
    Flush,
    Shutdown,
}

impl<T> Message<T> {
    pub fn unwrap(self) -> T {
        match self {
            Message::Write(t) => t,
            _ => panic!("This should never happen!!!"),
        }
    }
}

impl<T> Serialize for Message<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Message::Write(t) => t.serialize(serializer),
            Message::Shutdown => serializer.serialize_str("shutdown"),
            Message::Flush => serializer.serialize_str("flush"),
        }
    }
}

#[derive(Debug)]
pub struct Writer<T> {
    pub tx: mpsc::Sender<Message<T>>,
}

impl<T> Writer<T>
where
    T: Send + 'static + std::fmt::Debug + Serialize,
{
    pub fn new<F>(func: F, buffer_size: usize, batch_size: usize) -> Self
    where
        F: Fn(Vec<T>) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel(buffer_size);

        tokio::spawn(async move {
            tracing::info!("Starting writer");

            let mut buffer = Vec::with_capacity(batch_size);
            while let Some(msg) = rx.recv().await {
                match &msg {
                    Message::Shutdown => break,
                    Message::Flush => {
                        func(buffer.drain(..).map(|m: Message<T>| m.unwrap()).collect());
                    }
                    Message::Write(_) => {
                        buffer.push(msg);
                        if buffer.len() == batch_size {
                            func(buffer.drain(..).map(|msg| msg.unwrap()).collect());
                        }
                    }
                };
            }

            tracing::debug!("Shutting down writer");

            if !buffer.is_empty() {
                func(buffer.drain(..).map(|msg| msg.unwrap()).collect());
            }

            tracing::debug!("Writer shutdown");
        });

        Self { tx }
    }

    pub async fn write(&self, t: T) {
        match self.tx.send(Message::Write(t)).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to write: {}", e);
            }
        }
    }

    pub async fn write_many(&self, t: impl IntoIterator<Item = T>) {
        for t in t {
            self.write(t).await;
        }
    }

    pub async fn shutdown(&self) {
        match self.tx.send(Message::Shutdown).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to shutdown: {}", e);
            }
        };
    }

    pub async fn flush(&self) {
        match self.tx.send(Message::Flush).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to flush: {}", e);
            }
        };
    }
}
