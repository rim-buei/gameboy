mod gb;

use self::gb::GameBoy;
use self::gb::cartridge::Cartridge;
use self::gb::joypad::Button;
use self::gb::screen::{SCREEN_H, SCREEN_W};
use js_sys::Uint8Array;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlInputElement, HtmlSelectElement, ImageData, KeyboardEvent,
    ProgressEvent,
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

    handle_input(gameboy.clone())?;
    match handle_load_rom(gameboy.clone()) {
        Ok(()) => (),
        Err(msg) => web_sys::console::log_1(&msg),
    };
    match handle_select_rom(gameboy.clone()) {
        Ok(()) => (),
        Err(msg) => web_sys::console::log_1(&msg),
    };

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

fn handle_load_rom(gameboy: Rc<RefCell<GameBoy>>) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let load_rom = document
        .get_element_by_id("load-rom")
        .ok_or_else(|| JsValue::from_str("element with 'load-rom' not found, skipping"))?;

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
            let array = Uint8Array::new(&buffer);
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

    load_rom.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn handle_select_rom(gameboy: Rc<RefCell<GameBoy>>) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let select_rom = document
        .get_element_by_id("select-rom")
        .ok_or_else(|| JsValue::from_str("element with 'select-rom' not found, skipping"))?;

    let closure = Closure::wrap(Box::new(move |event: Event| {
        let select: HtmlSelectElement = event.target().unwrap().dyn_into().unwrap();
        let path = select.value();

        let window = window.clone();
        let gameboy = gameboy.clone();

        spawn_local(async move {
            let result = async {
                let response = JsFuture::from(window.fetch_with_str(&path)).await?;
                let response: web_sys::Response = response.dyn_into()?;

                if !response.ok() {
                    return Err(JsValue::from_str("failed to load ROM"));
                }

                let buffer = JsFuture::from(response.array_buffer()?).await?;
                let rom = Uint8Array::new(&buffer);

                let cart = Cartridge::new(rom.to_vec());
                gameboy.borrow_mut().pause();
                gameboy.borrow_mut().load(cart);
                gameboy.borrow_mut().unpause();
                Ok::<(), JsValue>(())
            }
            .await;

            if let Err(error) = result {
                web_sys::console::error_1(&error);
            }
        });
    }) as Box<dyn FnMut(Event)>);

    select_rom.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Trigger the "change" to load the initial rom
    select_rom.dispatch_event(&Event::new("change")?)?;

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
