use std::{
    env, io, fs
};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("usage: rlox [script]");
        return Ok(());
    } else if args.len() == 1 {
        runfile(&args[0]);
    } else {
        runPrompt()
    }

    Ok(())
}

fn runfile(path: &String) -> Result<(), io::Error> {
    let bytes = fs::
    Ok(())
}

fn runPrompt() {}
