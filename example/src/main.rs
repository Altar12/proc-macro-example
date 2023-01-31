use print_me::PrintMe;
use print_me_derive::PrintMe;

#[derive(PrintMe)]
struct Pokemon {
    id: u32,
    name: String,
}

#[derive(PrintMe)]
enum Region {
    Kanto,
    Jhoto,
    Hoenn,
    Sinnoh,
}
fn main() {
    Pokemon::print_me();
    Region::print_me();
}
