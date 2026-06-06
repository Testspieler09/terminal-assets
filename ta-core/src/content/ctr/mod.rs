use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use ta_render_engine::{
    Scene, SceneTarget,
    codec::VideoCodec,
    color::ColorConfig,
    models::{AspectPreset, FontSettings, Grid, ImageOutput, OutputConfig, VideoOutput},
};

pub struct CtrScene;

impl Scene for CtrScene {
    fn name(&self) -> &str {
        "ctr"
    }

    fn targets(&self) -> Vec<SceneTarget> {
        let font = FontSettings {
            font_path: "assets/font.ttf".into(),
            font_size: 16.0,
        };
        let cell = font.estimated_cell_size();

        vec![
            // Top screen - static image
            SceneTarget::new(
                OutputConfig::Image(ImageOutput::from_preset(AspectPreset::CtrTopScreen, cell)),
                font.clone(),
                ColorConfig::default(),
            ),
            // Bottom screen - short video
            SceneTarget::new(
                OutputConfig::Video(VideoOutput::new(
                    Grid::from_dimensions(AspectPreset::CtrBottomScreen.into(), cell),
                    30,
                    3 * 30,
                    VideoCodec::Gif,
                )),
                font.clone(),
                ColorConfig::default(),
            ),
        ]
    }

    fn draw(&self, target: &SceneTarget, frame: usize, area: Rect, buffer: &mut Buffer) {
        // differentiate layout per target via output type or dimensions
        match &target.output {
            OutputConfig::Image(_) => draw_top(area, buffer),
            OutputConfig::Video(_) => draw_bottom(frame, area, buffer),
        }
    }
}

fn draw_top(area: Rect, buffer: &mut Buffer) {
    Block::default()
        .title("CTR Top")
        .borders(Borders::ALL)
        .render(area, buffer);
}

fn draw_bottom(frame: usize, area: Rect, buffer: &mut Buffer) {
    Block::default()
        .title(format!("CTR Bottom - frame {frame}"))
        .borders(Borders::ALL)
        .render(area, buffer);
}
