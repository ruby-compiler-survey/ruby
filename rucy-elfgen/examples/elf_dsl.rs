extern crate mrusty;
extern crate rucy_elfgen;
use rucy_elfgen::models::*;
use rucy_libelf_sys::consts::*;
use std::path::Path;

const USAGE: &str = "Usage: cargo run --example elf_dsl -- [MRUBY_DSL_PATH] [DEST_PATH]";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script = std::env::args().nth(1).expect(USAGE);

    let dest = std::env::args().nth(2).expect(USAGE);

    let mruby = mrusty::Mruby::new();

    rucy_elfgen::eval_elf_dsl(&mruby, &Path::new(&script))?;
    let source = rucy_elfgen::copy_definition_to_rust(&mruby)?;

    rucy_elfgen::generate(dest, source)
}
