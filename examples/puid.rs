use puid::puid;

fn main() {
    println!("{}", puid!("pref"));
    println!("{}", puid!("pref", 24));
}
