use anyhow::Result;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

pub trait CreateCommand {
    fn command() -> Result<Command>;
}

pub async fn set_commands(
    client: Client,
    application_id: Id<ApplicationMarker>,
    commands: &[Command],
    guild_id: Id<GuildMarker>,
) -> Result<()> {
    let interaction_client = client.interaction(application_id);

    if cfg!(debug_assertions) {
        interaction_client
            .set_guild_commands(guild_id, commands)
            .await?;
    } else {
        interaction_client.set_global_commands(commands).await?;
    }

    Ok(())
}
