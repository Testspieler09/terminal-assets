use clap::Parser;
use ta_render_engine::Scene;

use crate::cli_args::RenderMode;

pub(crate) mod cli_args;
pub(crate) mod content;
pub(crate) mod tui;

fn main() -> Result<(), std::io::Error> {
    let args = cli_args::CliArgs::parse();
    let mode = args.mode();
    let all_scenes = content::all_scenes();

    let scenes: Vec<&Box<dyn Scene>> = match &mode {
        RenderMode::All => all_scenes.values().collect(),
        RenderMode::Scenes(names) => names.iter().filter_map(|n| all_scenes.get(n)).collect(),
        RenderMode::Interactive => match tui::run_scene_selector(&all_scenes)? {
            Some(names) if names.is_empty() => {
                println!("Nothing selected, nothing rendered.");
                return Ok(());
            }
            Some(names) => names.iter().filter_map(|n| all_scenes.get(n)).collect(),
            None => return Ok(()),
        },
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

    Ok(())
}
