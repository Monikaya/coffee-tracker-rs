use std::fs;
// FS interaction
use std::fs::create_dir;
use std::fs::remove_dir_all;
use std::fs::File;
//use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

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

    if !Path::new("data/coffeebase.json").is_file() {
        first_run_setup();
    }

    if DEBUG {
        let db_purge_result = debug_purge_data();
        println!(
            "Purged DB because we be in dev mode and I'm lazy.\nResult: {:?}",
            db_purge_result
        );
    }
}

fn debug_purge_data() -> std::io::Result<()> {
    remove_dir_all("data")?;
    Ok(())
}

fn init_data() -> std::io::Result<()> {
    create_dir("data")?;
    let mut file = File::create("data/coffeebase.json")?;
    file.write_all(b"{}")?;
    Ok(())
}

fn first_run_setup() {
    println!("Hey, welcome to my silly Rust coffee/caffeine tracker!");

    println!("First, we're going to create our files. If there are any errors we'll let you know.");
    let resp = init_data();
    eprintln!("{:?}", resp);
}
