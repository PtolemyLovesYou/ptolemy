use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Message<T> {
    Write(T),
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
                    _ => (),
                };

                buffer.push(msg);
                if buffer.len() == batch_size {
                    func(buffer.drain(..).map(|msg| msg.unwrap()).collect());
                }
            }

            tracing::info!("Shutting down writer");
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

    pub async fn shutdown(&self) -> Result<(), mpsc::error::SendError<Message<T>>> {
        self.tx.send(Message::Shutdown).await
    }
}
