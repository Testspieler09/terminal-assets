use std::{path::Path, process::Command};

use crate::models::VideoOutput;

#[derive(Clone)]
pub enum VideoCodec {
    H264 {
        /// Constant Rate Factor: 0 (lossless) - 51 (worst). Default 23.
        crf: Option<u8>,
    },
    Vp9 {
        /// Target bitrate e.g. "2M". None = use CRF mode (recommended).
        bitrate: Option<String>,
        /// CRF for VP9: 0 (best) - 63 (worst). Default 31.
        crf: Option<u8>,
    },
    Gif,
}

impl VideoCodec {
    /// The ffmpeg flags this codec produces, before any custom overrides.
    fn default_flags(&self) -> Vec<String> {
        match self {
            Self::H264 { crf } => vec![
                "-c:v".into(),
                "libx264".into(),
                "-crf".into(),
                crf.unwrap_or(23).to_string(),
                "-preset".into(),
                "slow".into(),
                // H.264 requires even dimensions; pad if needed
                "-vf".into(),
                "pad=ceil(iw/2)*2:ceil(ih/2)*2".into(),
                "-pix_fmt".into(),
                "yuv420p".into(),
            ],
            Self::Vp9 { bitrate, crf } => {
                let mut flags = vec![
                    "-c:v".into(),
                    "libvpx-vp9".into(),
                    "-crf".into(),
                    crf.unwrap_or(31).to_string(),
                ];
                match bitrate {
                    Some(b) => flags.extend(["-b:v".into(), b.clone()]),
                    None => flags.extend(["-b:v".into(), "0".into()]), // pure CRF mode
                }
                flags
            }
            Self::Gif => vec![
                // Two-pass GIF: palette generation + dither
                // (handled separately in encode(), see below)
            ],
        }
    }
}

#[derive(Debug)]
pub enum EncodeError {
    FfmpegNotFound,
    EncodeFailed { stderr: String },
    GifPaletteFailed { stderr: String },
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FfmpegNotFound => write!(f, "ffmpeg not found on PATH"),
            Self::EncodeFailed { stderr } => write!(f, "ffmpeg encode failed:\n{stderr}"),
            Self::GifPaletteFailed { stderr } => write!(f, "ffmpeg palette pass failed:\n{stderr}"),
        }
    }
}

impl std::error::Error for EncodeError {}

/// Merges a directory of sequentially named frames into a video.
///
/// # Arguments
/// - `frames_dir`   - directory containing frames named like `frame_0001.png`
/// - `output_path`  - destination file e.g. `out/video.mp4`
/// - `video`        - `VideoOutput` carrying fps + grid dimensions
/// - `codec`        - target format + quality settings
/// - `extra_flags`  - optional raw ffmpeg flags appended last (override defaults)
pub fn encode_video(
    frames_dir: &Path,
    output_path: &Path,
    video: &VideoOutput,
    codec: &VideoCodec,
    extra_flags: Option<&[&str]>,
) -> Result<(), EncodeError> {
    let input_pattern = frames_dir
        .join("frame_%04d.png")
        .to_string_lossy()
        .into_owned();

    match codec {
        VideoCodec::Gif => encode_gif(&input_pattern, output_path, video, extra_flags),
        _ => encode_standard(&input_pattern, output_path, video, codec, extra_flags),
    }
}

fn encode_standard(
    input_pattern: &str,
    output_path: &Path,
    video: &VideoOutput,
    codec: &VideoCodec,
    extra_flags: Option<&[&str]>,
) -> Result<(), EncodeError> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-framerate",
        &video.frames_per_second.to_string(),
        "-i",
        input_pattern,
    ]);
    for flag in codec.default_flags() {
        cmd.arg(flag);
    }
    if let Some(flags) = extra_flags {
        cmd.args(flags);
    }
    cmd.args(["-y", output_path.to_str().unwrap()]);

    run_ffmpeg(cmd).map_err(|stderr| EncodeError::EncodeFailed { stderr })
}

/// GIF needs a two-pass encode: palette generation then dither.
fn encode_gif(
    input_pattern: &str,
    output_path: &Path,
    video: &VideoOutput,
    extra_flags: Option<&[&str]>,
) -> Result<(), EncodeError> {
    let palette_path = output_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("_palette.png");

    // Pass 1: generate palette
    let mut palette_cmd = Command::new("ffmpeg");
    palette_cmd.args([
        "-framerate",
        &video.frames_per_second.to_string(),
        "-i",
        input_pattern,
        "-vf",
        "palettegen=stats_mode=full",
        "-y",
        palette_path.to_str().unwrap(),
    ]);
    run_ffmpeg(palette_cmd).map_err(|stderr| EncodeError::GifPaletteFailed { stderr })?;

    // Pass 2: encode with palette + dither
    let mut gif_cmd = Command::new("ffmpeg");
    gif_cmd.args([
        "-framerate",
        &video.frames_per_second.to_string(),
        "-i",
        input_pattern,
        "-i",
        palette_path.to_str().unwrap(),
        "-lavfi",
        "paletteuse=dither=bayer:bayer_scale=5",
    ]);
    if let Some(flags) = extra_flags {
        gif_cmd.args(flags);
    }
    gif_cmd.args(["-y", output_path.to_str().unwrap()]);
    run_ffmpeg(gif_cmd).map_err(|stderr| EncodeError::EncodeFailed { stderr })?;

    std::fs::remove_file(&palette_path).ok(); // clean up, non-fatal
    Ok(())
}

fn run_ffmpeg(mut cmd: Command) -> Result<(), String> {
    let output = cmd.output().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            "ffmpeg not found".to_string()
        } else {
            e.to_string()
        }
    })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}
