use std::{fs, process::Stdio};
use tokio::{io::AsyncReadExt, process::Command};
use crate::prelude::*;

static MAX_FILESIZE_MB: f32 = 10.0;

#[poise::command(prefix_command, slash_command, category = "Media")]
pub async fn video(
    ctx: Context<'_>,
    #[description = "Video URL"] url: String,
) -> Result<(), Error> {
    let file_name = jamcat_utils::gen_unique_filename();
    let file_path = format!("{}/{}", jamcat_core::constants::TMP_DIR, file_name);

    let mut handle = Command::new("yt-dlp")
        .args(["-4", &url, "-o", &file_path])
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
            wait_msg.delete(ctx).await?;
            return Err(Error::Generic(format!("Download failed: {err}")));
        }

        return Err(Error::Generic("Download failed".to_string()));
    }

    let mut file_path = None;

    let dir = fs::read_dir(jamcat_core::constants::TMP_DIR)?;
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

        if filesize > MAX_FILESIZE_MB {
            wait_msg.delete(ctx).await?;
            fs::remove_file(file_path)?;
            return Err(Error::Generic(format!(
                "Video is too big ({filesize:.2}MB / {:.2}MB) (Discord is cooked. Hawk tuah on Discord)",
                MAX_FILESIZE_MB
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