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
        // Let's draw a picture with Piet!

        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        ctx.fill(rect, &Self::BG_COLOR);

        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.

        let table = self.game.table();

        let box_width = Self::CELL_SIZE.width + Self::NET_BORDER_SIZE.width;
        let box_height = Self::CELL_SIZE.height + Self::NET_BORDER_SIZE.height;

        let x_start = Self::MARGIN.width;
        let x_end = x_start + box_width * table.width as f64;
        for horiz_offset in 0..table.height + 1 {
            let y = Self::MARGIN.height + box_height * horiz_offset as f64;
            let line = Line::new((x_start, y), (x_end, y));
            ctx.stroke(line, &Self::NET_COLOR, Self::NET_BORDER_SIZE.width)
        }

        let y_start = Self::MARGIN.height;
        let y_end = y_start + box_height * table.height as f64;
        for vert_offset in 0..table.width + 1 {
            let x = Self::MARGIN.width + box_width * vert_offset as f64;
            let line = Line::new((x, y_start), (x, y_end));
            ctx.stroke(line, &Self::NET_COLOR, Self::NET_BORDER_SIZE.height)
        }


        /*
                // Create an arbitrary bezier path
                let mut path = BezPath::new();
                path.move_to(Point::ORIGIN);
                path.quad_to((80.0, 90.0), (size.width, size.height));
                // Create a color
                let stroke_color = Color::rgb8(0, 128, 0);
                // Stroke the path with thickness 1.0
                ctx.stroke(path, &stroke_color, 1.0);

                // Rectangles: the path for practical people
                let rect = Rect::from_origin_size((10., 10.), (100., 100.));
                // Note the Color:rgba8 which includes an alpha channel (7F in this case)
                let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
                ctx.fill(rect, &fill_color);*/
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