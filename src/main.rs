use std::{sync::Arc, time::Duration};

use opcua::{
    client::{ClientBuilder, DataChangeCallback, IdentityToken, MonitoredItem, Session},
    crypto::SecurityPolicy,
    types::{
        DataValue, MessageSecurityMode, MonitoredItemCreateRequest, NodeId, StatusCode,
        TimestampsToReturn, UserTokenPolicy,
    },
};

fn values(data_value: &DataValue, item: &MonitoredItem) {
    let node_id = &item.item_to_monitor().node_id;
    if let Some(ref value) = data_value.value {
        println!("Item \"{}\", Value = {:?}", node_id, value);
    } else {
        println!(
            "Item \"{}\", Value not found, error: {}",
            node_id,
            data_value.status.as_ref().unwrap()
        );
    }
}
async fn subscription(session: Arc<Session>, ns: u16) -> Result<(), StatusCode> {
    let subscription_id = session
        .create_subscription(
            Duration::from_secs(1),
            10,
            30,
            0,
            0,
            true,
            DataChangeCallback::new(|dv, item| {
                println!("SubscriptionID");
                println!("Change from server:");
                values(&dv, item);
            }),
        )
        .await?;
    println!("Created PubSub SubscriptionID: {}", subscription_id);
    // Monitory DataItems
    //
    let items: Vec<MonitoredItemCreateRequest> = ["v1", "v2", "v3", "v4"]
        .iter()
        .map(|f| NodeId::new(ns, *f).into())
        .collect();
    let _ = session
        .create_monitored_items(subscription_id, TimestampsToReturn::Both, items)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let server_url = "opc.tcp://localhost:4840";
    let mut client = ClientBuilder::new()
        .application_name("Simple Client")
        .application_uri("urn:SimpleClient")
        .product_uri("urn:SimpleClient")
        .trust_server_certs(true)
        .create_sample_keypair(true)
        .session_timeout(5)
        .client()
        .unwrap();
    let (session, event_loop) = client
        .new_session_from_endpoint(
            (
                server_url.as_ref(),
                SecurityPolicy::None.to_str(),
                MessageSecurityMode::None,
                UserTokenPolicy::anonymous(),
            ),
            IdentityToken::Anonymous,
        )
        .await
        .unwrap();
    let handler = event_loop.spawn();
    session.wait_for_connection().await;

    // PubSub Connection to retrieve Values
    //
    if let Err(result) = subscription(session.clone(), 2).await {
        println!(
            "ERROR: Got an error while subscribing to variables - {}",
            result
        );
        let _ = session.disconnect().await;
    }

    handler.await.unwrap();
}
