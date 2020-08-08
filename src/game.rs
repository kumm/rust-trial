use crate::table::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Action {
    Put(Cell),
    Quit,
}

pub struct Game {
    table: Table,
    current: Figure,
    result: Option<GameResult>,
}

pub trait Player {
    fn step(&self, table: &Table, current: Figure) -> Action;
}

struct NoPlayer;

impl Player for NoPlayer {
    fn step(&self, _table: &Table, current: Figure) -> Action {
        panic!("Player \"{:?}\" undefined", current)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Error {
    GameIsOver,
    InvalidStep,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct TurnEvent {
    figure: Figure,
    action: Action,
    result: Option<GameResult>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GameResult {
    Drawn,
    Winner(Figure),
}

impl Game {
    pub fn new(table: Table, first_figure: Figure) -> Game {
        Game {
            table,
            current: first_figure,
            result: None,
        }
    }

    pub fn turn(&mut self, players: [&dyn Player; 2]) -> Result<TurnEvent, Error> {
        if self.result.is_some() {
            return Err(Error::GameIsOver);
        }
        let player = players[self.current as usize];
        let action = player.step(&self.table, self.current);
        match action {
            Action::Quit => {
                self.result = Some(GameResult::Winner(self.current.opponent()));
            }
            Action::Put(cell) => {
                let cell_value = &mut self.table[cell];
                if cell_value.is_some() {
                    return Err(Error::InvalidStep);
                }
                cell_value.replace(self.current);
            }
        };
        Ok(self.turn_over(action))
    }

    fn turn_over(&mut self, action: Action) -> TurnEvent {
        let turn_event = TurnEvent {
            action,
            figure: self.current,
            result: self.result,
        };
        if self.result.is_none() {
            self.current = self.current.opponent();
        }
        turn_event
    }

    pub fn figure_on_turn(&self) -> Figure {
        self.current
    }

    pub fn is_over(&self) -> bool {
        self.result.is_some()
    }

    pub fn table(&self) -> &Table {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlayer {
        action: Action,
    }

    impl Player for TestPlayer {
        fn step(&self, _table: &Table, _current: Figure) -> Action {
            self.action
        }
    }

    #[test]
    fn new_game() {
        let table = Table::new(10, 10);
        let game = Game::new(table, Figure::O);
        assert!(!game.is_over());
    }

    #[test]
    #[should_panic]
    fn start_with_no_player() {
        let table = Table::new(10, 10);
        let mut game = Game::new(table, Figure::O);
        game.turn([&NoPlayer; 2]);
    }

    #[test]
    fn turn_quit() {
        let table = Table::new(10, 10);
        let player_o = TestPlayer { action: Action::Quit };
        let mut game = Game::new(table, Figure::O);

        let mut players:[&dyn Player; 2] = [&NoPlayer; 2];
        players[Figure::O as usize] = &TestPlayer { action: Action::Quit };

        let result = game.turn(players);

        assert_eq!(Ok(TurnEvent {
            figure: Figure::O,
            action: Action::Quit,
            result: Some(GameResult::Winner(Figure::X)),
        }), result);
        assert!(game.is_over());

        players = [&NoPlayer; 2];
        players[Figure::X as usize] = &TestPlayer { action: Action::Quit };

        let error = game.turn(players);
        assert_eq!(Err(Error::GameIsOver), error);
    }

    #[test]
    fn turn2_put() {
        let table = Table::new(10, 10);
        let mut player_o = TestPlayer { action: Action::Put(Cell { col: 5, row: 5 }) };
        let mut player_x = TestPlayer { action: Action::Put(Cell { col: 5, row: 5 }) };
        let mut game = Game::new(table, Figure::X);

        let result = game.turn([&player_x, &player_o]);
        assert_eq!(Ok(TurnEvent {
            figure: Figure::X,
            action: Action::Put(Cell { col: 5, row: 5 }),
            result: None,
        }), result);
        assert!(!game.is_over());

        let error = game.turn([&player_x, &player_o]);
        assert_eq!(Err(Error::InvalidStep), error);
        assert!(!game.is_over());

        player_o.action = Action::Put(Cell { col: 1, row: 1 });
        let result = game.turn([&player_x, &player_o]);
        assert_eq!(Ok(TurnEvent {
            figure: Figure::O,
            action: Action::Put(Cell { col: 1, row: 1 }),
            result: None,
        }), result);
        assert!(!game.is_over());
    }
}
