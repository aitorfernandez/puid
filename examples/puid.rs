use puid::{errors::PuidError, Puid};

fn main() -> Result<(), PuidError> {
    // id with prefix foo_ and default entropy of 12 random characters at the end.
    let id = Puid::builder().prefix("foo")?.build()?;
    println!("{id}");

    // id with prefix bar_ and custom entropy of 24 random characters at the end.
    let id = Puid::builder().prefix("bar")?.entropy(24).build()?;
    println!("{id}");

    Ok(())
}
