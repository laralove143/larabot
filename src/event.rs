use anyhow::{bail, Result};
use tracing::{error, info};
use twilight_gateway::error::ReceiveMessageError;
use twilight_model::gateway::payload::incoming::Ready;

pub fn handle_ready(ready: &Ready) {
    info!("The bot is ready as {}.", ready.user.name);
}

pub fn handle_event_handle_error(err: &anyhow::Error) {
    error!(?err, "An error occurred while handling an event.");
}

pub fn handle_receive_message_error(err: ReceiveMessageError) -> Result<()> {
    if err.is_fatal() {
        error!(
            ?err,
            "A fatal error occurred while receiving an event. Exiting."
        );
        return Err(err.into());
    }

    error!(
        ?err,
        "A fatal error occurred while receiving an event, exiting"
    );
    Ok(())
}
