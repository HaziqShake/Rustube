use std::path::Path;
use std::process::Command;

pub fn mux_video_audio<P: AsRef<Path>>(video: P, audio: P, output: P) -> anyhow::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i").arg(video.as_ref())
        .arg("-i").arg(audio.as_ref())
        .arg("-c").arg("copy")
        .arg("-y")
        .arg(output.as_ref())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("FFmpeg muxing failed with status: {:?}", status)
    }
}
