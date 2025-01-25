use crate::{Context, Error};
use kilonova::{
    submissions::{Submissions, SubmissionsQuery},
    user::User,
};
use poise::serenity_prelude::model::mention::Mention;
use tokio::time;

/// Show this help menu
#[poise::command(prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
