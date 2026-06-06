use std::{collections::HashMap, io};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use ta_render_engine::Scene;

struct TuiState {
    items: Vec<SceneEntry>,
    list_state: ListState,
}

struct SceneEntry {
    name: String,
    meta: String,
    selected: bool,
}

impl SceneEntry {
    fn from_scene(scene: &dyn Scene) -> Self {
        let targets = scene.targets();
        let target_count = targets.len();
        let frame_total: usize = targets.iter().map(|t| t.frame_count()).sum();
        Self {
            name: scene.name().to_string(),
            meta: format!("{target_count} target(s), {frame_total} frame(s) total"),
            selected: false,
        }
    }
}

impl TuiState {
    fn new(scenes: Vec<&dyn Scene>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items: scenes.iter().map(|s| SceneEntry::from_scene(*s)).collect(),
            list_state,
        }
    }

    fn toggle_current(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.items[i].selected = !self.items[i].selected;
        }
    }

    fn toggle_all(&mut self) {
        let all_selected = self.items.iter().all(|i| i.selected);
        self.items
            .iter_mut()
            .for_each(|i| i.selected = !all_selected);
    }

    fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn move_down(&mut self) {
        let last = self.items.len().saturating_sub(1);
        let i = match self.list_state.selected() {
            Some(i) => (i + 1).min(last),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn selected_names(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|i| i.selected)
            .map(|i| i.name.clone())
            .collect()
    }
}

/// Runs the interactive scene selector.
///
/// Returns `Some(names)` of selected scenes, or `None` if the user
/// confirmed with nothing selected (caller should exit without rendering).
pub fn run_scene_selector(
    scenes: &HashMap<String, Box<dyn Scene>>,
) -> io::Result<Option<Vec<String>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = TuiState::new(scenes.values().map(|s| s.as_ref()).collect());
    let result = run_loop(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut TuiState,
) -> io::Result<Option<Vec<String>>> {
    loop {
        terminal.draw(|f| draw(f, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                KeyCode::Char(' ') => state.toggle_current(),
                KeyCode::Char('a') => state.toggle_all(),
                KeyCode::Enter => {
                    let selected = state.selected_names();
                    return Ok(Some(selected));
                }
                _ => {}
            }
        }
    }
}

fn draw(f: &mut ratatui::Frame, state: &mut TuiState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    let items: Vec<ListItem> = state
        .items
        .iter()
        .map(|entry| {
            let checkbox = if entry.selected { "[x]" } else { "[ ]" };
            let line = Line::from(vec![
                Span::styled(format!("{checkbox} "), Style::default().fg(Color::Blue)),
                Span::raw(&entry.name),
                Span::styled(
                    format!("  {}", entry.meta),
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Scenes "),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut state.list_state);

    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" ↑/k ", Style::default().fg(Color::Yellow)),
        Span::raw("up  "),
        Span::styled("↓/j ", Style::default().fg(Color::Yellow)),
        Span::raw("down  "),
        Span::styled("<space> ", Style::default().fg(Color::Yellow)),
        Span::raw("toggle  "),
        Span::styled("a ", Style::default().fg(Color::Yellow)),
        Span::raw("all  "),
        Span::styled("<enter> ", Style::default().fg(Color::Yellow)),
        Span::raw("confirm  "),
        Span::styled("q/esc ", Style::default().fg(Color::Yellow)),
        Span::raw("quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(hints, chunks[1]);
}
