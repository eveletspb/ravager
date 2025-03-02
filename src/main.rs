use std::time::Duration;

use common::print_message;
use docker_builder::process;

mod common;
mod finder;
mod docker_builder;

const TITLE :&str= r"
////////////////////////////////////////////                                 
///    _ _ __ ___ ____ _ __ _ ___ _ _    ///
///   | '_/ _` \ V / _` / _` / -_) '_|   ///
///   |_| \__,_|\_/\__,_\__, \___|_|     ///
///                     |___/            ///
////////////////////////////////////////////
";


fn main() {
    print_message(TITLE,Duration::from_micros(1));
    process();
}
