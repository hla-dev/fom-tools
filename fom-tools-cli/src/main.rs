extern crate clap;

use clap::App;
use fom_tools_lib;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    // let fom_filename = "modules/NETN-FOM-3.0-rc1/modules/NETN-BASE.xml";
    let fom_filename = "modules/RPR-FOM_v2.0/RPR-Base_v2.0.xml";
    if let Ok(fom_file) = File::open(fom_filename) {
        let _ = fom_tools_lib::parse(fom_file);
    }
}
