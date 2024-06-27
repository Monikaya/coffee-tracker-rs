// FS interaction
use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

fn debug_purge_json() -> std::io::Result<()> {
    remove_file("coffeebase.json")?;
    Ok(())
}

fn create_json() -> std::io::Result<()> {
    let mut file = File::create("coffeebase.json")?;
    file.write_all(b"{}")?;
    Ok(())
}

fn main() {
    /*
    First: we need to display what people have in their DB (json for now? probably. Something like sql would be nicer l8r).
    Then, we ask people if they'd like to add a new thing.
    After that, we ask for time (now or inputted time),
    Then what category (coffee/energy drink/tea), then more specific (espresso/RedBull small)
    Potentially extra info later (low priority)
    Then we just add it, say success (or err), then output the new table
    */

    const DEBUG: bool = true;

    println!("Hey, welcome to my silly Rust coffee/caffeine tracker!");

    let resp = create_json();
    eprintln!("{:?}", resp);

    if DEBUG {
        let db_purge = debug_purge_json();
        println!(
            "Purged DB because we be in dev mode and I'm lazy.\nResult: {:?}",
            db_purge
        );
    }
}
