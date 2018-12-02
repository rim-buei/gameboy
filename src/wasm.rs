#[macro_use]
extern crate stdweb;

mod gb;

use self::gb::cartridge::Cartridge;
use self::gb::screen::{SCREEN_H, SCREEN_W};
use self::gb::GameBoy;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web;
use stdweb::web::event::{ChangeEvent, ProgressLoadEvent};
use stdweb::web::html_element::{CanvasElement, InputElement};
use stdweb::web::{document, CanvasRenderingContext2d, FileList, FileReader, FileReaderResult};

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
                $y
        }
    };
}

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
    add_load_rom_event_handler(gameboy.clone());
    async_render_loop(ctx, gameboy.clone());

    stdweb::event_loop();
}

fn async_render_loop(ctx: CanvasRenderingContext2d, gameboy: Rc<RefCell<GameBoy>>) {
    web::window().request_animation_frame(move |_| {
        let screen = gameboy.borrow_mut().step();

        js! {
            @{&ctx}.putImageData(new ImageData(
                Uint8ClampedArray.from(@{screen}),
                @{SCREEN_W},
                @{SCREEN_H},
            ), 0, 0);
        }

        async_render_loop(ctx, gameboy);
    });
}

fn add_load_rom_event_handler(gameboy: Rc<RefCell<GameBoy>>) {
    let load_rom_button = web::document().get_element_by_id("load-rom").unwrap();
    load_rom_button.add_event_listener(move |event: ChangeEvent| {
        let input: InputElement = event.target().unwrap().try_into().unwrap();
        let files: FileList = js!( return @{input}.files; ).try_into().unwrap();
        let file = match files.iter().next() {
            Some(file) => file,
            None => return,
        };

        let reader = FileReader::new();
        reader.add_event_listener(enclose!([gameboy, reader] move |_: ProgressLoadEvent| {
            let rom: Vec<u8> = match reader.result().unwrap() {
                FileReaderResult::ArrayBuffer(buffer) => buffer,
                _ => unreachable!(),
            }
            .into();

            let cart = Cartridge::new(rom);
            gameboy.borrow_mut().pause();
            gameboy.borrow_mut().load(cart);
            gameboy.borrow_mut().unpause();
        }));

        reader.read_as_array_buffer(&file).unwrap();
    });
}
