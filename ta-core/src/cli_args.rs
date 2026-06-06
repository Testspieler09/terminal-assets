use std::path::PathBuf;

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(
    about = "Render the content you created",
    group(
        ArgGroup::new("mode")
            .args(["scenes", "interactive", "all"])
            .required(false)
    )
)]
pub(crate) struct CliArgs {
    #[arg(short, long, value_name = "SCENE")]
    /// The scenes to render, by name. Mutually exclusive with --interactive and --all.
    pub(crate) scenes: Vec<String>,

    #[arg(short, long)]
    /// Open an interactive TUI to select scenes. Mutually exclusive with --scenes and --all.
    pub(crate) interactive: bool,

    #[arg(short, long)]
    /// Render all scenes (default when no arguments are given).
    pub(crate) all: bool,

    #[arg(short, long, value_name = "DIR", required = true)]
    /// Directory to write rendered output into.
    pub(crate) output: PathBuf,
}

impl CliArgs {
    /// Resolves the effective mode, defaulting to All when no flag is given.
    pub(crate) fn mode(&self) -> RenderMode {
        if self.interactive {
            RenderMode::Interactive
        } else if !self.scenes.is_empty() {
            RenderMode::Scenes(self.scenes.clone())
        } else {
            // covers explicit --all and the no-args default
            RenderMode::All
        }
    }
}

#[derive(Debug)]
pub(crate) enum RenderMode {
    All,
    Interactive,
    Scenes(Vec<String>),
}
