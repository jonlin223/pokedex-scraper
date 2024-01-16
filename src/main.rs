use std::path::PathBuf;

use pokedex_scraper::scrape;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {

    let url = "https://pokemondb.net/pokedex/game/platinum";
    let game = "platinum";

    // Pick a game
    // Scrape page associated with that game - add game to dex json
        // For every entry
        // get name, type, image name
        // add under game in dex json
        // download the png if not already stored in the sprites folder
    // Storage Format
        // Pokedex (folder)
            // dex.json
                // game -> pokemon -> (type + sprite link)
            // sprites (folder)
                // (game)_(name).png

    scrape(url).await;
}
