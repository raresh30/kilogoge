use crate::{Context, Error};

/// List all handles
#[poise::command(prefix_command)]
pub async fn list_handles(ctx: Context<'_>) -> Result<(), Error> {
    let mut list = String::new();
    for (discord_name, kn_handle) in ctx.data().handles.lock().unwrap().iter() {
        list.push_str(format!("{} has the handle {}\n", discord_name, kn_handle).as_str());
    }
    ctx.say(list).await?;
    Ok(())
}
