mod gb;

use self::gb::GameBoy;
use self::gb::cartridge::Cartridge;
use self::gb::joypad::Button;
use self::gb::screen::{SCREEN_H, SCREEN_W};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlInputElement, ImageData, KeyboardEvent, ProgressEvent,
};

macro_rules! enclose {
    ([$($x: ident), *] $y: expr) => {
        {$(let $x = $x.clone();)* $y}
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();

    let gameboy = Rc::new(RefCell::new(GameBoy::new()));
    handle_custom_rom(gameboy.clone())?;
    handle_input(gameboy.clone())?;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.query_selector("canvas").unwrap().unwrap().dyn_into().unwrap();
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();

    async_render_loop(ctx, gameboy);

    Ok(())
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&info.to_string()));
    }));
}

fn handle_custom_rom(gameboy: Rc<RefCell<GameBoy>>) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let load_rom_button = document.get_element_by_id("load-rom").unwrap();

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let files = match input.files() {
            Some(files) => files,
            None => return,
        };
        let file = match files.get(0) {
            Some(file) => file,
            None => return,
        };

        let reader = web_sys::FileReader::new().unwrap();

        let onload = enclose!([gameboy, reader] Closure::wrap(Box::new(move |_: ProgressEvent| {
            let buffer = reader.result().unwrap();
            let array = js_sys::Uint8Array::new(&buffer);
            let rom: Vec<u8> = array.to_vec();

            let cart = Cartridge::new(rom);
            gameboy.borrow_mut().pause();
            gameboy.borrow_mut().load(cart);
            gameboy.borrow_mut().unpause();
        }) as Box<dyn FnMut(ProgressEvent)>));

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        reader.read_as_array_buffer(&file).unwrap();
    }) as Box<dyn FnMut(Event)>);

    load_rom_button.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn handle_input(gameboy: Rc<RefCell<GameBoy>>) -> Result<(), JsValue> {
    fn key_to_button(key: &str) -> Option<Button> {
        match key.to_lowercase().as_ref() {
            "x" => Some(Button::A),
            "z" => Some(Button::B),

            "enter" => Some(Button::Start),
            " " => Some(Button::Select),

            "arrowup" => Some(Button::Up),
            "arrowdown" => Some(Button::Down),
            "arrowleft" => Some(Button::Left),
            "arrowright" => Some(Button::Right),

            _ => None,
        }
    }

    let window = web_sys::window().unwrap();

    let keydown = enclose!([gameboy] Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if event.repeat() {
            return;
        }

        if let Some(button) = key_to_button(&event.key()) {
            gameboy.borrow_mut().press(button);
        }
    }) as Box<dyn FnMut(KeyboardEvent)>));
    window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
    keydown.forget();

    let keyup = enclose!([gameboy] Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(button) = key_to_button(&event.key()) {
            gameboy.borrow_mut().release(button);
        }
    }) as Box<dyn FnMut(KeyboardEvent)>));
    window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;
    keyup.forget();

    Ok(())
}

fn async_render_loop(ctx: CanvasRenderingContext2d, gameboy: Rc<RefCell<GameBoy>>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let screen = gameboy.borrow_mut().step();

        let image_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&screen), SCREEN_W as u32, SCREEN_H as u32).unwrap();
        ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
