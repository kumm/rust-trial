use druid::{AppLauncher, Color, Data, Env, Lens, LocalizedString, Point, Rect, Widget, WidgetExt, WindowDesc};
use druid::kurbo::*;
use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::Controller;
use druid::widget::prelude::*;

use crate::game::*;
use crate::table::*;

mod game;
mod table;

struct TableWidget {
    game: Game,
}


#[derive(Clone, Data, Lens)]
struct GameState {
    // #[druid(ignore)]
    //game: Game,
}

impl TableWidget {
    const CELL_PADDING: Size = Size::new(5.0, 5.0);
    const CELL_SIZE: Size = Size::new(32.0, 32.0);
    const BORDER_SIZE: Size = Size::new(2.0, 2.0);
    const FIG_LINE_WIDTH: f64 = 3.0;
    const O_COLOR: Color = Color::rgb8(0, 128, 0);
    const X_COLOR: Color = Color::rgb8(128, 0, 0);
    const BORDER_COLOR: Color = Color::rgb8(0, 0, 128);
    const BG_COLOR: Color = Color::WHITE;

    fn paint_table(&self, ctx: &mut PaintCtx, table: &Table) {
        let x_end = Self::CELL_SIZE.width * table.col_count() as f64;
        for horiz_offset in 0..table.row_count() + 1 {
            let y = Self::CELL_SIZE.height * horiz_offset as f64;
            let line = Line::new((0., y), (x_end, y));
            ctx.stroke(line, &Self::BORDER_COLOR, Self::BORDER_SIZE.width)
        }

        let y_end = Self::CELL_SIZE.height * table.row_count() as f64;
        for vert_offset in 0..table.col_count() + 1 {
            let x = Self::CELL_SIZE.width * vert_offset as f64;
            let line = Line::new((x, 0.), (x, y_end));
            ctx.stroke(line, &Self::BORDER_COLOR, Self::BORDER_SIZE.height)
        }

        for row in 0..table.row_count() {
            let row_values = &table[row];
            for col in 0..row_values.len() {
                row_values[col].map(|figure| self
                    .paint_figure(ctx, Cell { row, col }, figure));
            }
        }
    }

    fn paint_figure(&self, ctx: &mut PaintCtx, cell: impl Into<Rect>, figure: Figure) {
        let rect = cell.into();
        match figure {
            Figure::X => self.paint_x(ctx, &rect),
            Figure::O => self.paint_o(ctx, &rect),
        }
    }

    fn paint_x(&self, ctx: &mut PaintCtx, rect: &Rect) {
        let line1 = Line::new((rect.x0, rect.y0), (rect.x1, rect.y1));
        let line2 = Line::new((rect.x1, rect.y0), (rect.x0, rect.y1));
        ctx.stroke(line1, &Self::X_COLOR, Self::FIG_LINE_WIDTH);
        ctx.stroke(line2, &Self::X_COLOR, Self::FIG_LINE_WIDTH);
    }

    fn paint_o(&self, ctx: &mut PaintCtx, rect: &Rect) {
        let circle = Circle::new(rect.center(), rect.width() / 2.0);
        ctx.stroke(circle, &Self::O_COLOR, Self::FIG_LINE_WIDTH);
    }
}

impl Into<Rect> for Cell {
    fn into(self) -> Rect {
        let p0 = Point {
            x: TableWidget::CELL_SIZE.width * self.col as f64,
            y: TableWidget::CELL_SIZE.height * self.row as f64,
        };
        Rect::from_origin_size(p0, TableWidget::CELL_SIZE)
            .inflate(-TableWidget::CELL_PADDING.width, -TableWidget::CELL_PADDING.height)
    }
}

impl From<Point> for Cell {
    fn from(point: Point) -> Self {
        Cell {
            row: (point.y / TableWidget::CELL_SIZE.height) as usize,
            col: (point.x / TableWidget::CELL_SIZE.width) as usize,
        }
    }
}


impl Widget<GameState> for TableWidget {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, _data: &mut GameState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &GameState,
        _env: &Env,
    ) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &GameState, _data: &GameState, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &GameState,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &GameState, _env: &Env) {
        self.paint_table(ctx, self.game.table())
    }
}

struct TableController {

}

impl Controller<GameState, TableWidget> for TableController {
    fn event(&mut self, child: &mut TableWidget, ctx: &mut EventCtx, event: &Event, data: &mut GameState, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
            }
            Event::MouseUp(up_event) => {
                if !ctx.is_active() {
                    return;
                }
                ctx.set_active(false);
                if ctx.is_hot() {
                    let cell = Cell::from(up_event.pos);
                    let game = &mut child.game;
                    let table = game.table();
                    if cell.is_valid(table) {
                        game.set_player(game.figure_on_turn(), Box::new(UiPlayer { action: Action::Put(cell) }));
                        let result = game.turn();
                        if result.is_ok() {
                            ctx.request_paint();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

struct UiPlayer {
    action: Action,
}

impl Player for UiPlayer {
    fn step(&self, _table: &Table, _current: Figure) -> Action {
        self.action
    }
}


pub fn main() {
    let table = Table::new(10, 10);
    let game = Game::new(table, Figure::O);

    let controller = TableController {};
    let widget = TableWidget { game }
        .controller(controller)
        .padding((15.0))
        .background(Color::WHITE);

    let window = WindowDesc::new(|| widget).title(
        LocalizedString::new("window-title").with_placeholder("Gomoku"),
    );
    let game_state = GameState {};

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(game_state)
        .expect("launch failed");
}
