pub async fn receive() -> SignalEvent {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen signal");

    SignalEvent::Interrupted
}

#[derive(Debug)]
pub enum SignalEvent {
    /// INT (ctrl-c).
    Interrupted,
}
