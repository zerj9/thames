use crate::incoming::{IncomingMsgId, MessageHandler};
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

// Defined in the incoming module
pub struct DefaultMessageHandler;
impl MessageHandler for DefaultMessageHandler {}

pub struct Client {
    reader: Arc<tokio::sync::Mutex<tokio::io::ReadHalf<TcpStream>>>,
    writer: Arc<tokio::sync::Mutex<tokio::io::WriteHalf<TcpStream>>>,
    message_handler: Arc<dyn MessageHandler>,
}

impl Client {
    pub async fn new(address: &str) -> Result<Self> {
        Self::new_with_handler(address, DefaultMessageHandler).await
    }

    pub async fn new_with_handler<H: MessageHandler + 'static>(
        address: &str,
        handler: H,
    ) -> Result<Self> {
        let stream = TcpStream::connect(address)
            .await
            .context("Failed to connect to TWS")?;
        let (reader, writer) = tokio::io::split(stream);
        Ok(Self {
            reader: Arc::new(tokio::sync::Mutex::new(reader)),
            writer: Arc::new(tokio::sync::Mutex::new(writer)),
            message_handler: Arc::new(handler),
        })
    }

    pub async fn connect(&self) -> Result<()> {
        let mut writer = self.writer.lock().await;
        writer
            .write_all(b"API\0\0\0\0\x09v157..178")
            .await
            .context("Failed to send initial payload")?;
        println!("Initial payload sent successfully");

        // Read the response using the new reading logic
        let response = Self::read_message(&mut *self.reader.lock().await)
            .await
            .context("Failed to read initial response")?;

        let server_version = response[0].clone();
        let server_time = response[1].clone();

        println!("Server version: {server_version}");
        println!("Server time: {server_time}");

        // Send the StartApi message (code 71)
        drop(writer); // Release the lock on the writer
        self.send_message(vec!["71", "2", "1", ""])
            .await
            .context("Failed to send StartApi message")?;
        println!("StartApi message sent successfully");
        println!("Sleeping for 1 seconds...");
        sleep(Duration::from_secs(1)).await;

        println!("Starting reader:");
        Self::start_reader(self).await?;

        Ok(())
    }

    pub async fn send_message(&self, messages: Vec<&str>) -> Result<()> {
        // Concatenate all messages with a null byte appended to each
        let concatenated_message = messages.join("\0") + "\0";

        // Calculate the length of the concatenated message in bytes
        let length = concatenated_message.as_bytes().len();

        // Create a buffer to hold the length prefix and the message
        let mut full_payload = Vec::new();

        // Write the length as a 4-byte big-endian integer
        full_payload.extend_from_slice(&(length as u32).to_be_bytes());

        // Append the concatenated message
        full_payload.extend_from_slice(concatenated_message.as_bytes());

        // Send the message
        let mut writer = self.writer.lock().await;
        writer
            .write_all(&full_payload)
            .await
            .context("Failed to send message")?;

        println!("Message sent successfully: {:?}", messages);
        Ok(())
    }

    pub async fn start_reader(&self) -> Result<()> {
        let reader = self.reader.clone();
        let message_handler = self.message_handler.clone();
        tokio::spawn(async move {
            loop {
                let mut reader_guard = reader.lock().await;
                match Self::read_message(&mut *reader_guard).await {
                    Ok(parts) => {
                        let parts_slice: Vec<&str> = parts.iter().map(AsRef::as_ref).collect();
                        if let Err(e) = message_handler.handle_message(parts_slice) {
                            eprintln!("Error handling message: {}", e);
                            Self::print_message_parts(&parts);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading message: {}", e);
                        break;
                    }
                }
                // The lock is released here when reader_guard goes out of scope
            }
        });
        Ok(())
    }

    async fn read_message<T: AsyncReadExt + Unpin>(reader: &mut T) -> Result<Vec<String>> {
        let mut size_buffer = [0u8; 4];
        reader
            .read_exact(&mut size_buffer)
            .await
            .context("Failed to read message size")?;
        let size = u32::from_be_bytes(size_buffer);

        let mut message = vec![0u8; size as usize];
        reader
            .read_exact(&mut message)
            .await
            .context("Failed to read message content")?;

        // Split the message by null bytes and collect into a vector of strings
        let parts: Vec<String> = message
            .split(|&byte| byte == 0)
            .map(|part| String::from_utf8_lossy(part).into_owned())
            .collect();

        //Self::print_message_parts(&parts);
        Ok(parts)
    }

    fn print_message_parts(parts: &[String]) {
        if parts.is_empty() {
            println!("Received empty message");
            return;
        }

        let msg_type = &parts[0];
        let content = if parts.len() > 1 {
            parts[1..].join(", ")
        } else {
            String::new()
        };

        match msg_type.parse::<IncomingMsgId>() {
            Ok(enum_value) => println!("Message Type: {:?}, Content: [{}]", enum_value, content),
            Err(_) => println!("Unknown Message Type: {}, Content: [{}]", msg_type, content),
        }
    }
}
