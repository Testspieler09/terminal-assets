use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use ta_render_engine::{
    GlobalTargetId, Scene, SceneTarget,
    codec::VideoCodec,
    color::ColorScheme,
    models::{AspectPreset, FontSettings, Grid, ImageOutput, OutputConfig, VideoOutput},
};

pub struct CtrScene;

impl Scene for CtrScene {
    fn name(&self) -> &str {
        "ctr"
    }

    fn targets(&self) -> Vec<SceneTarget> {
        let font = FontSettings {
            font_path: "FiraMonoNerdFont-Regular.otf".into(),
            bold_font_path: None,
            font_size: 16.0,
        };
        let cell = font.estimated_cell_size();

        vec![
            SceneTarget::new(
                GlobalTargetId::CtrMain,
                OutputConfig::Image(ImageOutput::from_preset(AspectPreset::CtrTopScreen, cell)),
                font.clone(),
                ColorScheme::Dracula.into(),
            ),
            SceneTarget::new(
                GlobalTargetId::CtrSidebar,
                OutputConfig::Video(VideoOutput::new(
                    Grid::from_dimensions(AspectPreset::CtrBottomScreen.into(), cell),
                    30,
                    3 * 30,
                    VideoCodec::Gif,
                )),
                font.clone(),
                ColorScheme::Classic.into(),
            ),
        ]
    }

    fn draw(&self, target: &SceneTarget, frame: usize, area: Rect, buffer: &mut Buffer) {
        match &target.id {
            GlobalTargetId::CtrMain => draw_top(area, buffer),
            GlobalTargetId::CtrSidebar => draw_bottom(frame, area, buffer),
            _ => {}
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
