use chrono::{DateTime, Local};

#[derive(Debug, Default)]
struct App {
    time: DateTime<Local>,
    exit: bool,
}
/*
impl App {
    fn run(&mut self, mut term: DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.time = Local::now();
            term.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if !event::poll(Duration::from_secs(1))? {
            return Ok(());
        }
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}
*/
