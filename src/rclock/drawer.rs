use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line, Rectangle};

pub fn sep(context: &mut Context) {
    context.draw(&Rectangle {
        x: 2.5,
        y: 2.5,
        width: 5.0,
        height: 5.0,
        color: Color::Green,
    });

    context.draw(&Rectangle {
        x: 2.5,
        y: 8.5,
        width: 5.0,
        height: 5.0,
        color: Color::Green,
    });
}

pub fn zero(context: &mut Context) {
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 10.0,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: 5.0,
        y1: 0.0,
        x2: 5.0,
        y2: 10.0,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: 0.0,
        y1: 10.0,
        x2: 5.0,
        y2: 10.0,
        color: Color::Green,
    });
}
