use clap::Parser;
use ta_render_engine::Scene;

use crate::cli_args::RenderMode;

pub(crate) mod cli_args;
pub(crate) mod content;

fn main() {
    let args = cli_args::CliArgs::parse();
    let mode = args.mode();
    let all_scenes = content::all_scenes();

    let scenes: Vec<&Box<dyn Scene>> = match &mode {
        RenderMode::All => all_scenes.iter().collect(),
        RenderMode::Scenes(names) => all_scenes
            .iter()
            .filter(|s| names.contains(&s.name().to_string()))
            .collect(),
        RenderMode::Interactive => todo!("interactive TUI picker"),
    };

    for scene in scenes {
        for target in scene.targets() {
            for frame in 0..target.frame_count() {
                let buffer = scene.render_frame(&target, frame);
                // next step: rasterize buffer → RgbaImage via ab_glyph
                // then: write frame to args.output / scene.name() / frame_XXXX.png
                // then: encode via ffmpeg if VideoOutput
                let _ = (buffer, frame, &args.output);
            }
        }
    }
}
