use std::io::Stdout;
use std::time::{Duration};
use crossterm::{terminal, event, cursor, style, ExecutableCommand};
use crate::game::Game;
use crate::direction::Direction;
use crate::command::Command;
use crate::ui::UI;

#[derive(Debug)]
pub struct ConsoleUI {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
}

impl UI for ConsoleUI {
    fn init(&mut self, game: &Game) {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            .execute(terminal::SetSize(game.width + 3, game.height + 3)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(cursor::Hide).unwrap();
    }

    fn render(&mut self, game: &Game) {
        self.draw_borders(game);
        self.draw_background(game);
        self.draw_food(game);
        self.draw_snake(game);
    }

    fn shutdown(&mut self, game: &Game) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(terminal::SetSize(cols, rows)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(cursor::Show).unwrap()
            .execute(style::ResetColor).unwrap();
        terminal::disable_raw_mode().unwrap();
        
        println!("Game Over! Your score is {}", game.score);
    }

    fn get_command(&self, wait_for: Duration) -> Option<crate::command::Command> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            event::KeyCode::Char('q') | event::KeyCode::Char('Q') | event::KeyCode::Esc => Some(Command::Quit),
            event::KeyCode::Char('c') | event::KeyCode::Char('C') =>
                if key_event.modifiers == event::KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                }
            event::KeyCode::Up => Some(Command::Turn(Direction::Up)),
            event::KeyCode::Right => Some(Command::Turn(Direction::Right)),
            event::KeyCode::Down => Some(Command::Turn(Direction::Down)),
            event::KeyCode::Left => Some(Command::Turn(Direction::Left)),
            _ => None
        }
    }
}

impl ConsoleUI {
    pub fn new(stdout: Stdout) -> Self {
        let original_terminal_size: (u16, u16) = terminal::size().unwrap();
        Self {
            stdout,
            original_terminal_size,
        }
    }

    fn draw_borders(&mut self, game: &Game) {
        self.stdout.execute(style::SetForegroundColor(style::Color::DarkGrey)).unwrap();

        for y in 0..game.height + 2 {
            self.stdout
                .execute(cursor::MoveTo(0, y)).unwrap()
                .execute(style::Print("#")).unwrap()
                .execute(cursor::MoveTo(game.width + 1, y)).unwrap()
                .execute(style::Print("#")).unwrap();
        }

        for x in 0..game.width + 2 {
            self.stdout
                .execute(cursor::MoveTo(x, 0)).unwrap()
                .execute(style::Print("#")).unwrap()
                .execute(cursor::MoveTo(x, game.height + 1)).unwrap()
                .execute(style::Print("#")).unwrap();
        }

        self.stdout
            .execute(cursor::MoveTo(0, 0)).unwrap()
            .execute(style::Print("#")).unwrap()
            .execute(cursor::MoveTo(game.width + 1, game.height + 1)).unwrap()
            .execute(style::Print("#")).unwrap()
            .execute(cursor::MoveTo(game.width + 1, 0)).unwrap()
            .execute(style::Print("#")).unwrap()
            .execute(cursor::MoveTo(0, game.height + 1)).unwrap()
            .execute(style::Print("#")).unwrap();
    }

    fn draw_background(&mut self, game: &Game) {
        self.stdout.execute(style::ResetColor).unwrap();

        for y in 1..game.height + 1 {
            for x in 1..game.width + 1 {
                self.stdout
                    .execute(cursor::MoveTo(x, y)).unwrap()
                    .execute(style::Print(" ")).unwrap();
            }
        }
    }

    fn draw_food(&mut self, game: &Game) {
        self.stdout.execute(style::SetForegroundColor(style::Color::White)).unwrap();

        for food in game.food.iter() {
            self.stdout
                .execute(cursor::MoveTo(food.x + 1, food.y + 1)).unwrap()
                .execute(style::Print("•")).unwrap();
        }
    }

    fn draw_snake(&mut self, game: &Game) {
        let fg = style::SetForegroundColor(match game.speed % 3 {
            0 => style::Color::Green,
            1 => style::Color::Cyan,
            _ => style::Color::Yellow
        });

        self.stdout.execute(fg).unwrap();

        let body_points = game.snake.get_body_points();
        for (i, body) in body_points.iter().enumerate() {
            let previous = if i == 0 { None } else { body_points.get(i - 1) };
            let next = body_points.get(i + 1);
            let symbol = if let Some(&next) = next {
                if let Some(&previous) = previous {
                    if previous.x == next.x {
                        '║'
                    } else if previous.y == next.y {
                        '═'
                    } else {
                        let d = body.transform(Direction::Down, 1);
                        let r = body.transform(Direction::Right, 1);
                        let u = if body.y == 0 { body.clone() } else { body.transform(Direction::Up, 1) };
                        let l = if body.x == 0 { body.clone() } else { body.transform(Direction::Left, 1) };
                        if (next == d && previous == r) || (previous == d && next == r) {
                            '╔'
                        } else if (next == d && previous == l) || (previous == d && next == l) {
                            '╗'
                        } else if (next == u && previous == r) || (previous == u && next == r) {
                            '╚'
                        } else {
                            '╝'
                        }
                    }
                } else {
                    'O'
                }
            } else if let Some(&previous) = previous {
                if body.y == previous.y {
                    '═'
                } else {
                    '║'
                }
            } else {
                panic!("Invalid snake body point.");
            };

            self.stdout
                .execute(cursor::MoveTo(body.x + 1, body.y + 1)).unwrap()
                .execute(style::Print(symbol)).unwrap();
        }
    }
    
    fn wait_for_key_event(&self, wait_for: Duration) -> Option<event::KeyEvent> {
        if event::poll(wait_for).ok()? {
            let event = event::read().ok()?;
            if let event::Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
    }
}