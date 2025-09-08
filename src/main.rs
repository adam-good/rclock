use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn render(frame: &mut Frame) {
    frame.render_widget("hello w0rld", frame.area());
}

fn run(mut term: DefaultTerminal) -> Result<()> {
    loop {
        term.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}


