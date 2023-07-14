mod messages_generator;

use crate::messages_generator::MessagesGenerator;
use anyhow::Result;
use clap::Parser;
use iggy::client::{MessageClient, StreamClient, TopicClient};
use iggy::client_provider;
use iggy::client_provider::ClientProviderConfig;
use iggy::clients::client::{IggyClient, IggyClientConfig};
use iggy::messages::send_messages::{KeyKind, Message, SendMessages};
use iggy::streams::create_stream::CreateStream;
use iggy::streams::get_stream::GetStream;
use iggy::topics::create_topic::CreateTopic;
use samples::shared::args::Args;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    info!(
        "Producer has started, selected transport: {}",
        args.transport
    );
    let client_provider_config = Arc::new(ClientProviderConfig::from_args(args.to_sdk_args())?);
    let client = client_provider::get_client(client_provider_config).await?;
    let client = IggyClient::new(client, IggyClientConfig::default());
    let stream = client
        .get_stream(&GetStream {
            stream_id: args.stream_id,
        })
        .await;
    if stream.is_err() {
        info!("Stream does not exist, creating...");
        client
            .create_stream(&CreateStream {
                stream_id: args.stream_id,
                name: "sample".to_string(),
            })
            .await?;
        client
            .create_topic(&CreateTopic {
                stream_id: args.stream_id,
                topic_id: args.topic_id,
                partitions_count: args.partition_id,
                name: "orders".to_string(),
            })
            .await?;
    }

    produce_messages(&args, &client).await
}

async fn produce_messages(args: &Args, client: &IggyClient) -> Result<(), Box<dyn Error>> {
    info!(
        "Messages will be sent to stream: {}, topic: {}, partition: {} with interval {} ms.",
        args.stream_id, args.topic_id, args.partition_id, args.interval
    );
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(args.interval));
    let mut message_generator = MessagesGenerator::new();
    loop {
        let mut messages = Vec::new();
        let mut serializable_messages = Vec::new();
        for _ in 0..args.messages_per_batch {
            let serializable_message = message_generator.generate();
            // You can send the different message types to the same partition, or stick to the single type.
            let json_envelope = serializable_message.to_json_envelope();
            let message = Message::from_str(&json_envelope)?;
            messages.push(message);
            // This is used for the logging purposes only.
            serializable_messages.push(serializable_message);
        }
        client
            .send_messages(&SendMessages {
                stream_id: args.stream_id,
                topic_id: args.topic_id,
                key_kind: KeyKind::PartitionId,
                key_value: args.partition_id,
                messages_count: messages.len() as u32,
                messages,
            })
            .await?;
        info!("Sent messages: {:#?}", serializable_messages);
        interval.tick().await;
    }
}
