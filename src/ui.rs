use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" RClock ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .padding(Padding::new(0, 0, area.height / 2, 0));

        let timer_text = Text::from(self.time.format("%H:%M:%S").to_string().yellow());
        Paragraph::new(timer_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
