#[macro_use]
extern crate stdweb;

mod gb;

use self::gb::screen::Screen;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, set_timeout, CanvasRenderingContext2d};

fn main() {
    stdweb::initialize();

    let screen = Rc::new(RefCell::new(Screen::new()));

    let canvas: CanvasElement = document()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    async_render_loop(ctx, screen.clone());

    stdweb::event_loop();
}

fn async_render_loop(ctx: CanvasRenderingContext2d, screen: Rc<RefCell<Screen>>) {
    set_timeout(
        move || {
            let array = screen.borrow().dump();

            js! {
                @{&ctx}.putImageData(new ImageData(
                    Uint8ClampedArray.from(@{array}),
                    @{screen.borrow().width()},
                    @{screen.borrow().height()},
                ), 0, 0);
            }

            async_render_loop(ctx, screen);
        },
        1000 / 60, // 60 FPS
    );
}
