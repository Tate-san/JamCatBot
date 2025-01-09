use std::{fs, path::PathBuf, process::Stdio};

use tokio::{io::AsyncReadExt, process::Command};

use crate::{constants, utils};

use super::prelude::*;

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn yapper(
    ctx: Context<'_>,
    #[description = "Who's the yapper"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send_message(Message::Other(format!(
        "{} is the biggest yapper of em all",
        user.name
    )))
    .await?;

    user.direct_message(
        ctx.http(),
        serenity::CreateMessage::new().content("Quit with the yapping"),
    )
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn video(
    ctx: Context<'_>,
    #[description = "Video URL"] url: String,
) -> Result<(), Error> {
    let file_name = utils::generate_filename();
    let file_path = format!("{}/{}", constants::TMP_DIR, file_name);

    let mut handle = Command::new("yt-dlp")
        .args([&url, "-o", &file_path])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| Error::Generic("yt-dlp is not installed".to_string()))?;

    let wait_msg = ctx
        .send_message(Message::Other(
            "Just a second, i'm getting the sauce".to_string(),
        ))
        .await?;

    let status = handle.wait().await?;

    if !status.success() {
        if let Some(mut stderr) = handle.stderr {
            let mut err = String::new();
            stderr.read_to_string(&mut err).await?;
            return Err(Error::Generic(format!("Download failed: {err}")));
        }

        return Err(Error::Generic("Download failed".to_string()));
    }

    let mut file_path = None;

    let dir = fs::read_dir(constants::TMP_DIR)?;
    for item in dir {
        let item = item?;

        if item.file_name().into_string().unwrap().contains(&file_name) {
            file_path = Some(item.path());
            break;
        }
    }

    if let Some(file_path) = file_path {
        let buffer = fs::read(&file_path)?;

        let filesize = buffer.len() as f32 / 1000000.0;

        if filesize > constants::MAX_FILESIZE_MB {
            fs::remove_file(file_path)?;
            return Err(Error::Generic(format!(
                "Video is too big ({filesize:.2}MB / {:.2}MB)",
                constants::MAX_FILESIZE_MB
            )));
        }

        let _ = ctx
            .send(
                poise::CreateReply::default()
                    .attachment(serenity::CreateAttachment::bytes(
                        buffer,
                        format!("{file_name}.mp4"),
                    ))
                    .content(format!("Sent by {}", ctx.author().name)),
            )
            .await;

        wait_msg.delete(ctx).await?;

        fs::remove_file(file_path)?;
    } else {
        return Err(Error::Generic(
            "Video not properly downloaded, something went wrong".to_string(),
        ));
    }

    Ok(())
}
