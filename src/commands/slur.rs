use crate::{Context, Error};

use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Basically a ping command
#[poise::command(slash_command)]
pub(crate) async fn slur(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let file = File::open("quotes.txt")
        .unwrap_or_else(|_e| panic!("Quote file missing.")); // Open the quotes file
    let file = BufReader::new(file); // Read the quotes file
    let quotes = file.lines()
        .map(|res| res.expect("Failed to read line."));
    let quote = quotes.choose(&mut rand::thread_rng())
        .expect("No lines in file."); // Pick a random quote

    ctx.say(quote).await?;
    Ok(())
}
