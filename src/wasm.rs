#[macro_use]
extern crate stdweb;

mod gb;

use self::gb::cartridge::Cartridge;
use self::gb::joypad::Button;
use self::gb::screen::{SCREEN_H, SCREEN_W};
use self::gb::GameBoy;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web;
use stdweb::web::event::{ChangeEvent, KeyDownEvent, KeyUpEvent, ProgressLoadEvent};
use stdweb::web::html_element::{CanvasElement, InputElement};
use stdweb::web::{document, CanvasRenderingContext2d, FileList, FileReader, FileReaderResult};

macro_rules! enclose {
    ([$($x: ident), *] $y: expr) => {
        {$(let $x = $x.clone();)* $y}
    }
}

fn handle_custom_rom(gameboy: Rc<RefCell<GameBoy>>) {
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

fn handle_input(gameboy: Rc<RefCell<GameBoy>>) {
    let handler = |key: &str| -> Option<Button> {
        match key.to_lowercase().as_ref() {
            "z" => Some(Button::B),
            "x" => Some(Button::A),

            "enter" => Some(Button::Start),
            "shift" => Some(Button::Select),

            "arrowup" => Some(Button::Up),
            "arrowdown" => Some(Button::Down),
            "arrowleft" => Some(Button::Left),
            "arrowright" => Some(Button::Right),

            _ => None,
        }
    };

    web::window().add_event_listener(enclose!([gameboy] move |event: KeyDownEvent| {
        if event.repeat() {
            return;
        }

        match handler(&event.key()) {
            Some(button) => {
                gameboy.borrow_mut().press(button);
            }
            None => (),
        }
    }));
    web::window().add_event_listener(enclose!([gameboy] move |event: KeyUpEvent| {
        match handler(&event.key()) {
            Some(button) => {
                gameboy.borrow_mut().release(button);
            }
            None => (),
        }
    }));
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

fn main() {
    stdweb::initialize();

    let gameboy = Rc::new(RefCell::new(GameBoy::new()));
    handle_custom_rom(gameboy.clone());
    handle_input(gameboy.clone());

    let canvas: CanvasElement = document()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
    async_render_loop(ctx, gameboy.clone());

    stdweb::event_loop();
}
