extern crate lalrpop;

fn main() {
    if let Err(error) = lalrpop::process_root() {
        panic!("Failed to process LALRPOP files: {error}");
    }
}
