use crate::rclock::app;
//use crate::rclock::drawer;
use crate::rclock::pomodoro::Pomodoro;
use crate::rclock::pomodoro::PomodoroState;
use crate::rclock::pomodoro::TimerIntent;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use std::io;
use std::time::Duration;
use tui_big_text::{BigText, PixelSize};

pub struct UI {
    terminal: DefaultTerminal,
}

impl UI {
    pub fn new() -> Self {
        UI {
            terminal: ratatui::init(),
        }
    }

    pub fn shutdown(&self, app: &mut app::App) {
        app.stop();
        ratatui::restore();
    }

    pub fn toggle_pause(&self, app: &mut app::App) {
        app.toggle_pause();
    }

    pub fn view(&mut self, app: &app::App) {
        self.terminal
            .draw(|frame| {
                let (top, left, right) = UI::partition_layout(frame);
                UI::render_top_widget(frame, top, app);
                UI::render_left_widget(frame, left, app);
                UI::render_right_widget(frame, right, app);
            })
            .expect("EEP");
    }

    pub fn handle_events(&mut self, app: &mut app::App) -> io::Result<()> {
        match event::poll(Duration::from_millis(10))? {
            false => return Ok(()),
            true => {}
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, app);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut app::App) {
        match key_event.code {
            KeyCode::Char('q') => self.shutdown(app),
            KeyCode::Char(' ') => self.toggle_pause(app),
            _ => {}
        }
    }

    fn partition_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(40)])
            .split(frame.area());
        let (top, bottom) = (outer_layout[0], outer_layout[1]);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(bottom);
        let (left, right) = (inner_layout[0], inner_layout[1]);

        (top, left, right)
    }

    fn render_top_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        let time_str: String = app.base_time.time().format("%H:%M").to_string();
        let clock_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Color::Green)
            .centered()
            .lines(vec![time_str.into()])
            .build();
        frame.render_widget(block, area);

        let [area] = Layout::vertical([Constraint::Percentage(60)])
            .flex(Flex::Center)
            .areas(area);
        frame.render_widget(clock_text, area);
    }

    // TODO: Make this show a timer
    fn render_left_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);

        let pomodoro: Option<&Pomodoro> = app.get_pomodoro();

        let default = "None";
        let pomodoro_round_str: String = pomodoro
            .map(|p: &Pomodoro| -> String { p.get_round().to_string() })
            .unwrap_or(String::from(default));
        let pomodoro_timer_str: String = pomodoro
            .map(|p: &Pomodoro| -> String { p.get_timer().time().format("%H:%M:%S").to_string() })
            .unwrap_or(String::from(default));
        let pomodoro_state_str: String = pomodoro
            .map(|p: &Pomodoro| -> String {
                match p.get_state() {
                    PomodoroState::Running => "Running".to_string(),
                    PomodoroState::Paused => "Paused".to_string(),
                }
            })
            .unwrap_or(String::from(default));
        let timer_intent_str: String = pomodoro
            .map(|p: &Pomodoro| -> String {
                match p.get_intent() {
                    TimerIntent::Work => "Work".to_string(),
                    TimerIntent::Break => "Break".to_string(),
                }
            })
            .unwrap_or(String::from(default));

        // TODO: Better syntax??
        let msg = format!(
            "Round: {}\n{}\n{}\n{}",
            pomodoro_round_str, pomodoro_timer_str, pomodoro_state_str, timer_intent_str
        );
        frame.render_widget(Paragraph::new(msg).block(block), area);
    }

    fn render_right_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);

        let perc = app
            .get_pomodoro()
            .map(|p: &Pomodoro| p.get_timer().get_perc().floor())
            .unwrap_or(0.0);

        let gauge = Gauge::default()
            .block(block)
            .gauge_style(Color::Green)
            .percent(perc as u16);

        frame.render_widget(gauge, area);
    }
}
