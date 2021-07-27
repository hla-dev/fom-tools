extern crate clap;

use clap::App;
use fom_tools_lib;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let netn_base = "modules/NETN-FOM-3.0-rc1/modules/NETN-BASE.xml";
    if let Ok(netn_base_file) = File::open(netn_base) {
        let _ = fom_tools_lib::parse(netn_base_file);
    }
}
