// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    use sausage_factory::*;

    sausage_factory::make_sausage();
}
