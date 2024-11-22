use std::{sync::Arc, time::Duration};

use opcua::{
    client::{ClientBuilder, DataChangeCallback, IdentityToken, MonitoredItem, Session},
    crypto::SecurityPolicy,
    types::{
        DataValue, MessageSecurityMode, MonitoredItemCreateRequest, NodeId, StatusCode,
        TimestampsToReturn, UserTokenPolicy,
    },
};

async fn discover_servers(discovery_url: &str) -> Result<Vec<String>, StatusCode> {
    println!(
        "Attempting to connect to discovery server {} ...",
        discovery_url
    );
    // Create client to discover servers
    let mut client = opcua::client::Client::new(opcua::client::ClientConfig::new(
        "DiscoveryClient",
        "urn:DiscoveryClient",
    ));
    // Function to find servers
    match client.find_servers(discovery_url).await {
        Ok(servers) => {
            println!("Discovery server responded with {} servers:", servers.len());
            let mut discovered_servers = Vec::new(); //Getting all servers
            for server in servers {
                println!("Server: {}", server.application_name);
                if let Some(ref discovery_urls) = server.discovery_urls {
                    for url in discovery_urls {
                        println!("  Discovery URL: {}", url);
                        // Convert UAString to String
                        discovered_servers.push(url.as_ref().to_string());
                    }
                } else {
                    println!("  No discovery URLs for this server");
                }
            }
            Ok(discovered_servers)
        }
        Err(err) => {
            println!("ERROR: Cannot find servers on discovery server: {:?}", err);
            Err(err)
        }
    }
}

fn values(data_value: &DataValue, item: &MonitoredItem) {
    let node_id = &item.item_to_monitor().node_id;
    // Printing values from servers
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
    // getting SubscriptionID from server
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
    let discover_server = "opc.tpc://localhost:4840";
    match discover_servers(discover_server).await {
        Ok(servers) if !servers.is_empty() => {
            let server_url = servers[0].clone();
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
            if let Err(result) = subscription(session.clone(), 2).await {
                println!("Error, got an issue while Subscribing {}", result);
                let _ = session.disconnect().await;
            };
            handler.await.unwrap();
        }
        Ok(_) => {
            println!("No servers found at the discovery URL")
        }
        Err(err) => {
            println!("Discovery Failed: {:?}", err);
        }
    }
}
