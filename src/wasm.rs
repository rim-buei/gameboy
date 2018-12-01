#[macro_use]
extern crate stdweb;

mod gb;

use self::gb::screen::{SCREEN_H, SCREEN_W};
use self::gb::GameBoy;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let gameboy = Rc::new(RefCell::new(GameBoy::new()));
    async_render_loop(ctx, gameboy.clone());

    stdweb::event_loop();
}

fn async_render_loop(ctx: CanvasRenderingContext2d, gameboy: Rc<RefCell<GameBoy>>) {
    web::window().request_animation_frame(move |_| {
        gameboy.borrow_mut().step();
        let array = gameboy.borrow_mut().screen();

        js! {
            @{&ctx}.putImageData(new ImageData(
                Uint8ClampedArray.from(@{array}),
                @{SCREEN_W},
                @{SCREEN_H},
            ), 0, 0);
        }

        async_render_loop(ctx, gameboy);
    });
}
