use clap::Parser;
use ta_render_engine::{
    Scene,
    codec::{VideoCodec, encode_video},
    font_cache::FontCache,
    models::OutputConfig,
    raster::Rasterizer,
};

use crate::cli_args::RenderMode;

pub(crate) mod cli_args;
pub(crate) mod content;
pub(crate) mod tui;

fn main() -> anyhow::Result<()> {
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

    for scene in &scenes {
        let scene_out = args.output.join(scene.name());
        std::fs::create_dir_all(&scene_out)?;

        let mut font_cache = FontCache::default();

        for (target_index, target) in scene.targets().iter().enumerate() {
            let target_dir = scene_out.join(format!("target_{target_index:02}"));
            std::fs::create_dir_all(&target_dir)?;

            let font = font_cache.load_and_insert_font(target.font.font_path.clone())?;
            let rasterizer = Rasterizer::new(&target.font, font);

            for frame in 0..target.frame_count() {
                let buffer = scene.render_frame(target, frame);
                let img = rasterizer.rasterize(&buffer, &target.colors);

                let frame_path = target_dir.join(format!("frame_{frame:04}.png"));
                img.save(&frame_path)?;
            }

            // encode video if needed
            if let OutputConfig::Video(ref video) = target.output {
                let ext = match &video.codec {
                    VideoCodec::H264 { .. } => "mp4",
                    VideoCodec::Vp9 { .. } => "webm",
                    VideoCodec::Gif => "gif",
                };
                let output_file = scene_out.join(format!("target_{target_index:02}.{ext}"));
                encode_video(&target_dir, &output_file, video, &video.codec, None)?;
            }
        }
    }

    Ok(())
}
