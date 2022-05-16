mod evolve;
mod layout;

use evolve::optimise;

use std::io;

use crate::layout::DEFAULT_LAYOUT;

fn main() -> io::Result<()> {
    let layout = DEFAULT_LAYOUT;
    eprintln!("{}", serde_json::to_string_pretty(&layout)?);

    let optimised = optimise(layout)?;
    println!("{}", serde_json::to_string_pretty(&optimised)?);
    Ok(())
}
