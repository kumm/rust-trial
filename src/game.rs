use crate::table::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Action {
    Put(Cell),
    Quit,
}

pub struct Game {
    table: Table,
    players: [Box<dyn Player>; 2],
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
            players: [Box::new(NoPlayer), Box::new(NoPlayer)],
            current: first_figure,
            result: None,
        }
    }

    pub fn turn(&mut self) -> Result<TurnEvent, Error> {
        if self.result.is_some() {
            return Err(Error::GameIsOver);
        }
        let player = &self.players[self.current as usize];
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

    pub fn is_over(&self) -> bool {
        self.result.is_some()
    }

    pub fn set_player(&mut self, figure: Figure, player: Box<dyn Player>) {
        self.players[figure as usize] = player;
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
        game.turn();
    }

    #[test]
    fn turn_quit() {
        let table = Table::new(10, 10);
        let player_o = TestPlayer { action: Action::Quit };
        let mut game = Game::new(table, Figure::O);
        game.set_player(Figure::O, Box::new(player_o));

        let result = game.turn();

        assert_eq!(Ok(TurnEvent {
            figure: Figure::O,
            action: Action::Quit,
            result: Some(GameResult::Winner(Figure::X)),
        }), result);
        assert!(game.is_over());

        let error = game.turn();
        assert_eq!(Err(Error::GameIsOver), error);
    }

    #[test]
    fn turn_put() {
        let table = Table::new(10, 10);
        let player_o = TestPlayer { action: Action::Put(Cell { col: 5, row: 5 }) };
        let player_x = TestPlayer { action: Action::Put(Cell { col: 5, row: 5 }) };
        let mut game = Game::new(table, Figure::X);
        game.set_player(Figure::O, Box::new(player_o));
        game.set_player(Figure::X, Box::new(player_x));

        let result = game.turn();
        assert_eq!(Ok(TurnEvent {
            figure: Figure::X,
            action: Action::Put(Cell { col: 5, row: 5 }),
            result: None,
        }), result);
        assert!(!game.is_over());

        let error = game.turn();
        assert_eq!(Err(Error::InvalidStep), error);
        assert!(!game.is_over());

        let x = TestPlayer { action: Action::Put(Cell { col: 1, row: 1 }) };
        game.set_player(Figure::O, Box::new(x));
        let result = game.turn();
        assert_eq!(Ok(TurnEvent {
            figure: Figure::O,
            action: Action::Put(Cell { col: 1, row: 1 }),
            result: None,
        }), result);
        assert!(!game.is_over());
    }
}
