use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use ta_render_engine::{
    Scene, SceneTarget,
    color::ColorConfig,
    models::{AspectPreset, FontSettings, ImageOutput, OutputConfig},
};

pub struct T09Scene;

impl Scene for T09Scene {
    fn name(&self) -> &str {
        "t09"
    }

    fn targets(&self) -> Vec<SceneTarget> {
        let font = FontSettings {
            font_path: "FiraMonoNerdFont-Regular.otf".into(),
            font_size: 16.0,
        };
        let cell = font.estimated_cell_size();

        vec![SceneTarget::new(
            OutputConfig::Image(ImageOutput::from_preset(AspectPreset::SixteenToNine, cell)),
            font.clone(),
            ColorConfig::default(),
        )]
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
        .title("T09 Top")
        .borders(Borders::ALL)
        .render(area, buffer);
}

fn draw_bottom(frame: usize, area: Rect, buffer: &mut Buffer) {
    Block::default()
        .title(format!("T09 Bottom - frame {frame}"))
        .borders(Borders::ALL)
        .render(area, buffer);
}
