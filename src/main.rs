// FS interaction
use std::fs::create_dir;
use std::fs::remove_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// User Input
use text_io::read;

fn main() {
    /*
        Plan:
        First we need to display what people have in their DB (json for now? probably. Something like sql would be nicer l8r).
        Then, we ask people if they'd like to add a new thing.
        After that, we ask for time (now or inputted time),
        Then what category (coffee/energy drink/tea), then more specific (espresso/RedBull small)
        Potentially extra info later (low priority)
        Then we just add it, say success (or err), then output the new table
    */

    /*
        Just use this in order to reset database at the end of a run.
        This can be toggled on and off so I don't have to manually delete if I need to repetitively test stuff.
    */
    const DEBUG: bool = true;

    //Run Setup if data/ doesn't exist
    if !Path::new("data/coffeebase.json").is_file() {
        first_run_setup();
    }

    loop {
        println!("
            What would you like to do?\n
            1) Display your database.\n
            2) Add an entry.\n
            3) Modify an entry.\n
            4) Exit program.\n
        ");
        let choice: i32 = read!();

        match choice {
            1 => database_to_string(),
            2 => add_db_entry(),
            3 => modify_db_entry(),
            4 => break,
            _ => println!("Invalid number, please try again."),
        }
    }

    //Purge the database if the DEBUG value is true.
    if DEBUG {
        let db_purge_result = debug_purge_data();
        println!(
            "Purged DB because we be in dev mode and I'm lazy.\nResult: {:?}",
            db_purge_result
        );
    }
}

fn database_to_string() {
    println!("Doesn't exist yet.");
}

fn add_db_entry() {
    println!("Doesn't exist yet.");
}

fn modify_db_entry() {
    println!("Def doesn't exist yet :P");
}

/*
    Run if data directory does not exist. Run the user through the starting bits.
    Possibly ask for name, colors (eventually)
    Mostly automated though, just have to get things setup.
    Will also return errors if there are any (there shouldn't be any).
*/
fn first_run_setup() {
    println!("Hey, welcome to my silly Rust coffee/caffeine tracker!");

    println!("First, we're going to create our files. If there are any errors we'll let you know.");
    let resp = init_data();
    eprintln!("{:?}", resp);
}

/*
    Initialize *ata/ directory and data/coffeebase.json
*/
fn init_data() -> std::io::Result<()> {
    create_dir("data")?;
    let mut file = File::create("data/coffeebase.json")?;
    file.write_all(b"{}")?;
    Ok(())
}


fn debug_purge_data() -> std::io::Result<()> {
    remove_dir_all("data")?;
    Ok(())
}
