#[macro_use]
extern crate stdweb;

mod gb;

fn main() {
    stdweb::initialize();

    let message = "Hello, World!";
    js! {
        console.log( @{message} );
    }

    stdweb::event_loop();
}
