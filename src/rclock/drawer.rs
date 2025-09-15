use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line, Rectangle};

pub fn sep(context: &mut Context, area: Rect) {
    let width: f64 = f64::from(area.width);
    let height: f64 = f64::from(area.height);
    let x_center: f64 = width / 2.0;
    let y_center_low: f64 = 0.5 * height - 0.25 * height - 0.1 * height;
    let y_center_high: f64 = 0.5 * height + 0.25 * height + 0.1 * height;
    let sep_width: f64 = f64::max(width / 4.0, 2.0);
    let sep_height: f64 = f64::max(height / 4.0, 2.0);

    context.draw(&Rectangle {
        x: x_center - sep_width / 2.0,
        y: y_center_low - sep_height / 2.0,
        width: sep_width,
        height: sep_height,
        color: Color::Green,
    });

    context.draw(&Rectangle {
        x: x_center - sep_width / 2.0,
        y: y_center_high - sep_height / 2.0,
        width: sep_width,
        height: sep_height,
        color: Color::Green,
    });
}

pub fn zero(context: &mut Context, area: Rect) {
    let width: f64 = f64::from(area.width);
    let height: f64 = f64::from(area.height);
    let x_center: f64 = width / 2.0;
    let y_center: f64 = height / 2.0;
    let digit_width: f64 = 0.9 * width;
    let digit_height: f64 = 0.99 * height;
    let half_width: f64 = digit_width / 2.0;
    let half_height: f64 = digit_height / 2.0;

    context.draw(&Line {
        x1: x_center - half_width,
        y1: y_center - half_height,
        x2: x_center - half_width,
        y2: y_center + half_height,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: x_center + half_width,
        y1: y_center - half_height,
        x2: x_center + half_width,
        y2: y_center + half_height,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: x_center - half_width,
        y1: y_center + half_height,
        x2: x_center + half_width,
        y2: y_center + half_height,
        color: Color::Green,
    });

    context.draw(&Line {
        x1: x_center - half_width,
        y1: y_center - half_height,
        x2: x_center + half_width,
        y2: y_center - half_height,
        color: Color::Green,
    });
}
