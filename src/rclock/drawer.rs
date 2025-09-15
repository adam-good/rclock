use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::Points;
use ratatui::widgets::canvas::{Context, Line, Rectangle};

pub fn sep(context: &mut Context) {
    context.draw(&Points {
        coords: &[(0.0, 0.0), (5.0, 5.0)],
        color: Color::Red,
    });

    context.draw(&Points {
        coords: &[(2.0, 1.0), (3.0, 1.0), (2.0, 3.0), (3.0, 3.0)],
        color: Color::Green,
    });
}

pub fn zero(context: &mut Context) {
    context.draw(&Rectangle {
        x: 0.0,
        y: 0.0,
        width: 4.0,
        height: 4.0,
        color: Color::Green,
    });
}
