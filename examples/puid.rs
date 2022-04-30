use puid::puid;

fn main() {
    // id with prefix foo_ and default size of 12 random characters at the end.
    println!("{}", puid!("foo"));
    // id with prefix bar_ and custom size of 24 random characters at the end.
    println!("{}", puid!("bar", 24));
}
