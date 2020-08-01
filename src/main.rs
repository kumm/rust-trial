mod game;
mod table;

use crate::game::*;
use crate::table::*;

use druid::kurbo::Line;
use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, Rect, Point, Color};

struct TableWidget {
    game: Game,
}


#[derive(Clone, Data, Lens)]
struct GameState {
    // #[druid(ignore)]
    //game: Game,
}

impl TableWidget {
    const MARGIN: Size = Size::new(15.0, 15.0);
    const CELL_SIZE: Size = Size::new(30.0, 30.0);
    const NET_BORDER_SIZE: Size = Size::new(2.0, 2.0);
    const NET_COLOR: Color = Color::rgb8(0, 0, 128);
    const BG_COLOR: Color = Color::WHITE;

    fn paint_figure(&self, ctx: &mut PaintCtx, col: u8, row: u8, figure: Figure) {}

    fn paint_table(&self, ctx: &mut PaintCtx, table: &Table) {
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        ctx.fill(rect, &Self::BG_COLOR);
        let box_width = Self::CELL_SIZE.width + Self::NET_BORDER_SIZE.width;
        let box_height = Self::CELL_SIZE.height + Self::NET_BORDER_SIZE.height;

        let x_start = Self::MARGIN.width;
        let x_end = x_start + box_width * table.col_count() as f64;
        for horiz_offset in 0..table.row_count() + 1 {
            let y = Self::MARGIN.height + box_height * horiz_offset as f64;
            let line = Line::new((x_start, y), (x_end, y));
            ctx.stroke(line, &Self::NET_COLOR, Self::NET_BORDER_SIZE.width)
        }

        let y_start = Self::MARGIN.height;
        let y_end = y_start + box_height * table.row_count() as f64;
        for vert_offset in 0..table.col_count() + 1 {
            let x = Self::MARGIN.width + box_width * vert_offset as f64;
            let line = Line::new((x, y_start), (x, y_end));
            ctx.stroke(line, &Self::NET_COLOR, Self::NET_BORDER_SIZE.height)
        }

        for r in 0..table.row_count() {
            let row = &table[r];
            for c in 0..row.len() {
                row[c].map(|figure|
                    self.paint_figure(ctx, c as u8, r, figure));
            }
        }
    }
}

trait Painter {
    fn paint(&self, ctx: &mut PaintCtx);
}

impl Widget<GameState> for TableWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut GameState, _env: &Env) {}

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

pub fn main() {
    let table = Table::new(10, 10);
    let game = Game::new(table, Figure::O);

    let window = WindowDesc::new(|| TableWidget { game }).title(
        LocalizedString::new("window-title").with_placeholder("Gomoku"),
    );
    let game_state = GameState {};

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(game_state)
        .expect("launch failed");
}
