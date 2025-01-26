use crate::{Context, Error};
use kilonova::{
    submissions::{Submissions, SubmissionsQuery},
    user::User,
};
use poise::serenity_prelude::model::mention::Mention;
use tokio::time;

enum IdentifyStatus {
    Success,
    Failed,
}

/// Link your Kilonova handle to your Discord account
#[poise::command(prefix_command)]
pub async fn identify(
    ctx: Context<'_>,
    #[description = "Your Kilonova handle"] handle: String,
) -> Result<(), Error> {
    let problem_id = 1;
    let problem_url = format!("https://kilonova.ro/problems/{}", problem_id);
    ctx.say(format!(
        "{}, send a compile error to {} in the next 60 seconds",
        Mention::from(ctx.author().id),
        problem_url
    ))
    .await?;

    let time_in_seconds = 60;
    let period = 10; // Checks for submissions every 10 seconds
    let mut interval = time::interval(time::Duration::from_secs(period));
    let mut status = IdentifyStatus::Failed;
    let timestamp = chrono::offset::Utc::now().timestamp();
    for _i in 0..(time_in_seconds / period) {
        interval.tick().await;
        let sent_compile_error = check_submissions(problem_id, &handle, timestamp).await?;
        if sent_compile_error {
            status = IdentifyStatus::Success;
            let discord_name = ctx.author().name.clone();
            ctx.data()
                .handles
                .lock()
                .unwrap()
                .entry(discord_name)
                .or_insert(handle);
            break;
        }
    }
    match status {
        IdentifyStatus::Success => {
            ctx.say(format!(
                "{}, your Kilonova account was successfully linked.",
                Mention::from(ctx.author().id),
            ))
            .await?
        }
        IdentifyStatus::Failed => {
            ctx.say(format!(
                "{}, your Kilonova account couldn't be linked.",
                Mention::from(ctx.author().id),
            ))
            .await?
        }
    };

    Ok(())
}

async fn check_submissions(
    problem_id: i32,
    handle: &str,
    command_timestamp: i64,
) -> Result<bool, Error> {
    let user_id = User::by_name(handle).await?.id;
    let submissions = Submissions::get(SubmissionsQuery {
        problem_id: Some(problem_id),
        user_id: Some(user_id),
        ..Default::default()
    })
    .await?;
    if submissions.submissions.len() == 0 {
        return Ok(false);
    }
    let last_submission = &submissions.submissions[0];
    let submission_timestamp =
        chrono::DateTime::parse_from_str(&last_submission.created_at, "%Y-%m-%dT%H:%M:%S.%f%z")?
            .timestamp();
    Ok(last_submission.compile_error && submission_timestamp >= command_timestamp)
}
