use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Rectangle};

pub fn zero(context: &mut Context) {
    context.draw(&Rectangle {
        x: 1.0,
        y: 1.0,
        width: 20.0,
        height: 5.0,
        color: Color::Green,
    })
}
