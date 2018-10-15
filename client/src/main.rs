extern crate empholite_client;
extern crate yew;

use empholite_client::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
