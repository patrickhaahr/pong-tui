use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Menu,
    Game,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Menu
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    running: bool,
    mode: Mode,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        match self.mode {
            Mode::Menu => {
                let title = Line::from("Pong Game\n").bold().blue().centered();
                let text =
                    "Press `Esc`, `Ctrl-C` or `q` to stop running.\nPress `Enter` to start game";

                frame.render_widget(
                    Paragraph::new(text)
                        .block(Block::bordered().title(title))
                        .centered(),
                    frame.area(),
                );
            }
            Mode::Game => {
                Self::center_line(frame, frame.area());
            }
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Enter) => self.start_game(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn start_game(&mut self) {
        self.mode = Mode::Game;
    }

    fn center_line(frame: &mut Frame, area: Rect) {
        if area.height < 1 || area.width < 1 {
            return;
        }

        let center_col = ((area.width.saturating_sub(1) / 2) as usize).min(area.width as usize);

        let mut lines: Vec<String> = Vec::with_capacity(area.height as usize);
        for _ in 0..(area.height as usize) {
            let mut s = String::new();
            s.push_str(&" ".repeat(center_col as usize));
            s.push('█');
            lines.push(s);
        }

        let text = lines.join("\n");
        let p = Paragraph::new(text).style(Style::default());
        frame.render_widget(p, area);
    }
}
