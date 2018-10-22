#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello, World!";
    js! {
        console.log( @{message} );
    }

    stdweb::event_loop();
}
