use crate::debug;
use crate::models::Flashcard;
use crate::spa::{NaiveExponentialSPA, SPA};
use crate::session::Session;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, text::Line, widgets::*};
use std::{
    cell::RefCell,
    io::{self, stdout},
    rc::Rc,
};

use crate::db::Db;

#[derive(Clone, Copy)]
enum Review {
    Question,
    Answer,
}

#[derive(Clone, Copy)]
enum CurrentScreen {
    Index,
    FlashcardsReview(Review),
}

struct App {
    db: Rc<RefCell<Db>>,
    current_screen: CurrentScreen,
    session: Option<Session>,
}

impl App {
    fn new(db: Db) -> Self {
        App {
            db: Rc::new(RefCell::new(db)),
            current_screen: CurrentScreen::Index,
            session: None,
        }
    }

    async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let spa = NaiveExponentialSPA::new(50, 128);

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match self.current_screen {
                    CurrentScreen::Index => {
                        if key.code == KeyCode::Char('q') {
                            break;
                        } else if key.code == KeyCode::Char('r') {
                            let cards: Vec<Flashcard> = self
                                .db
                                .borrow_mut()
                                .get_cards()
                                .await
                                .unwrap()
                                .into_iter()
                                .collect();
                            let mut to_review_cards = vec![];
                            // TODO: filter requires async closure and it can get weird.
                            for card in cards {
                                let answers = self.db.borrow_mut().get_answers(&card).await.unwrap();
                                if spa.repeat_question(&answers) {
                                    to_review_cards.push(card);
                                }
                            }
                            let session = Session::new(to_review_cards);
                            self.session = Some(session);
                            self.current_screen = CurrentScreen::FlashcardsReview(Review::Question);
                        }
                    }
                    CurrentScreen::FlashcardsReview(state) => {
                        if key.code == KeyCode::Char('q') {
                            self.current_screen = CurrentScreen::Index;
                        }

                        match state {
                            Review::Question => {
                                if key.code == KeyCode::Char(' ') {
                                    self.current_screen =
                                        CurrentScreen::FlashcardsReview(Review::Answer);
                                }
                            }
                            Review::Answer => {
                                if key.code == KeyCode::Char(' ') {
                                    self.current_screen =
                                        CurrentScreen::FlashcardsReview(Review::Question);
                                }

                                let answer = match key.code {
                                    KeyCode::Char('g') => 100,
                                    KeyCode::Char('b') => 0,
                                    _ => continue,
                                };

                                debug!("rating answer: {}", answer);

                                let session = self
                                    .session
                                    .as_mut()
                                    .expect("Session must exist when doing reviews.");
                                let current_question = session.current_question().unwrap();
                                let answer = current_question.answer(answer);
                                self.db
                                    .borrow_mut()
                                    .persist_answer(answer)
                                    .await
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                                session.advance_questions();
                                self.current_screen =
                                    CurrentScreen::FlashcardsReview(Review::Question);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        match &mut self.current_screen {
            CurrentScreen::Index => {
                let text = Text::from(vec![
                    Line::from(Span::styled(
                        "Menu",
                        Style::default().bold().fg(Color::Gray),
                    )),
                    Line::from("Review flashcards - 'r'"),
                    Line::from("Quit application  - 'q'"),
                ]);
                frame.render_widget(
                    Paragraph::new(text)
                        .block(Block::default().title("Flashcards").borders(Borders::ALL)),
                    frame.size(),
                );
            }
            CurrentScreen::FlashcardsReview(state) => {
                let session = self
                    .session
                    .as_mut()
                    .expect("Session must exist when doing reviews.");
                let current_question = session.current_question();
                if current_question.is_none() {
                    frame.render_widget(
                        Paragraph::new("No card remaining for today review.").block(
                            Block::default()
                                .title("Flashcards review")
                                .borders(Borders::ALL),
                        ),
                        frame.size(),
                    );
                    return;
                }

                let current_question = current_question.unwrap();
                let text = match state {
                    Review::Question => current_question.question.as_str(),
                    Review::Answer => current_question.answer.as_str(),
                };
                frame.render_widget(
                    Paragraph::new(text).block(
                        Block::default()
                            .title("Flashcards review")
                            .borders(Borders::ALL),
                    ),
                    frame.size(),
                );
            }
        }
    }
}

pub async fn tui(db: Db) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new(db);

    app.run(&mut terminal).await?;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
