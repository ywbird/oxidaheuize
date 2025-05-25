use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                let outter_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(frame.area());

                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                    .split(outter_layout[0]);

                self.draw(frame, &outter_layout[1]);
                self.draw(frame, &layout[0]);
                self.draw(frame, &layout[1]);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, &rect: &Rect) {
        frame.render_widget(self, rect);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
            Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]),
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
