use crate::rclock::app;
use crate::rclock::drawer;
use chrono::Timelike;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::canvas::Context;
use std::io;
use std::time::Duration;

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

    pub fn view(&mut self, app: &app::App) {
        self.terminal
            .draw(|frame| {
                let (top, left, right) = UI::partition_layout(frame);
                UI::render_top_widget(frame, top, app);
                UI::render_left_widget(frame, left, app);
                UI::render_right_widget(frame, right, app);

                let (digit1, digit2, sep, digit3, digit4) = UI::partition_clock_layout(top);
                UI::render_clock(frame, digit1, digit2, sep, digit3, digit4, app);
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
            _ => {}
        }
    }

    fn partition_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(frame.area());
        let (top, bottom) = (outer_layout[0], outer_layout[1]);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(bottom);
        let (left, right) = (inner_layout[0], inner_layout[1]);

        (top, left, right)
    }

    fn partition_clock_layout(area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let digit_height: u16 = 5;
        let digit_width: u16 = 6;

        let vert_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(digit_height),
                Constraint::Fill(1),
            ])
            .split(area);
        let middle = vert_split[1];

        let horz_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1), //Percentage(15),
                Constraint::Length(digit_width),
                Constraint::Length(digit_width),
                Constraint::Length(digit_width),
                Constraint::Length(digit_width),
                Constraint::Length(digit_width),
                Constraint::Fill(1),
            ])
            .split(middle);
        let (digit1, digit2, sep, digit3, digit4) = (
            horz_split[1],
            horz_split[2],
            horz_split[3],
            horz_split[4],
            horz_split[5],
        );

        (digit1, digit2, sep, digit3, digit4)
    }

    fn render_clock(
        frame: &mut Frame,
        digit1: Rect,
        digit2: Rect,
        sep: Rect,
        digit3: Rect,
        digit4: Rect,
        app: &app::App,
    ) {
        let h = app.base_time.hour();
        let m = app.base_time.minute();
        let (d1, d2) = UI::time_to_digit_fn(h);
        let (d3, d4) = UI::time_to_digit_fn(m);
        UI::render_digit(frame, digit1, d1);
        UI::render_digit(frame, digit2, d2);
        UI::render_digit(frame, sep, drawer::sep);
        UI::render_digit(frame, digit3, d3);
        UI::render_digit(frame, digit4, d4);
    }

    fn time_to_digit_fn(time_like: u32) -> (impl Fn(&mut Context), impl Fn(&mut Context)) {
        let digit_fn_low = drawer::get(time_like % 10);
        let digit_fn_high = drawer::get(time_like / 10);

        (digit_fn_high, digit_fn_low)
    }

    fn render_digit(frame: &mut Frame, area: Rect, val: impl Fn(&mut Context)) {
        let left: f64 = 0.0;
        let right: f64 = f64::from(area.width);
        let bottom: f64 = 0.0;
        let top: f64 = f64::from(area.height);
        let widget = Canvas::default()
            //.block(Block::bordered())
            .marker(Marker::Block)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(val);

        frame.render_widget(widget, area);
    }

    fn render_top_widget(frame: &mut Frame, area: Rect, _app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        frame.render_widget(block, area);
    }

    // TODO: Make this show a timer
    fn render_left_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        let msg = app.base_time.format("%H:%M:%S").to_string();
        frame.render_widget(Paragraph::new(msg).block(block), area);
    }

    // TODO: Make this show a progress bar
    fn render_right_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        //let msg = app.base_time.format("%H:%M:%S").to_string();
        let msg = match app.get_primary_timer() {
            Some(t) => t.time().format("%H:%M:%S").to_string(),
            None => String::from("00:00:00"),
        };

        frame.render_widget(Paragraph::new(msg).block(block), area);
    }

    /*
        fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
            let [area] = Layout::horizontal([horizontal])
                .flex(Flex::Center)
                .areas(area);
            let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

            area
        }
    */
}
