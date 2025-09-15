use ratatui::style::Color;
use ratatui::widgets::canvas::Points;
use ratatui::widgets::canvas::{Context, Line, Rectangle};

pub fn get(val: u32) -> impl Fn(&mut Context) {
    match val {
        0 => zero,
        1 => one,
        2 => two,
        3 => three,
        4 => four,
        5 => five,
        6 => six,
        7 => seven,
        8 => eight,
        9 => nine,
        _ => sep,
    }
}
pub fn sep(context: &mut Context) {
    /*
    context.draw(&Points {
        coords: &[(0.0, 0.0), (5.0, 5.0)],
        color: Color::Red,
    });
    */

    context.draw(&Points {
        coords: &[(2.0, 1.0), (3.0, 1.0), (2.0, 3.0), (3.0, 3.0)],
        color: Color::Green,
    });
}

fn zero(context: &mut Context) {
    context.draw(&Rectangle {
        x: 0.0,
        y: 0.0,
        width: 5.0,
        height: 5.0,
        color: Color::Green,
    });
}

fn one(context: &mut Context) {
    context.draw(&Line {
        x1: 5.0,
        y1: 0.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
}

fn two(context: &mut Context) {
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 5.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 2.0,
        x2: 0.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 2.0,
        x2: 0.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
}

fn three(context: &mut Context) {
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 2.0,
        x2: 5.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
}

fn four(context: &mut Context) {
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 0.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 2.0,
        x2: 5.0,
        y2: 2.0,
        color: Color::Green,
    });
}

fn five(context: &mut Context) {
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 0.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 2.0,
        x2: 0.0,
        y2: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 2.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
}

fn six(context: &mut Context) {
    context.draw(&Rectangle {
        x: 0.0,
        y: 0.0,
        width: 5.0,
        height: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 0.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 0.0,
        y2: 5.0,
        color: Color::Green,
    });
}

fn seven(context: &mut Context) {
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
}

fn eight(context: &mut Context) {
    context.draw(&Line {
        x1: 5.0,
        y1: 5.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 5.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 2.0,
        x2: 5.0,
        y2: 2.0,
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
        y1: 0.0,
        x2: 0.0,
        y2: 5.0,
        color: Color::Green,
    });
}

fn nine(context: &mut Context) {
    context.draw(&Line {
        x1: 5.0,
        y1: 0.0,
        x2: 5.0,
        y2: 5.0,
        color: Color::Green,
    });
    context.draw(&Rectangle {
        x: 0.0,
        y: 2.0,
        width: 5.0,
        height: 2.0,
        color: Color::Green,
    });
    context.draw(&Line {
        x1: 0.0,
        y1: 0.0,
        x2: 5.0,
        y2: 0.0,
        color: Color::Green,
    });
}
