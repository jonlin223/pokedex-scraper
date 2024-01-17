use pokedex_scraper::scrape;

#[tokio::main]
async fn main() {

    // TODO Just note that you have to edit the name of the file yourself
    let url = "https://pokemondb.net/pokedex/game/black-white";

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

    match scrape(url).await {
        Some(pokedex) => {
            println!("{:?}", pokedex);
            let json = serde_json::to_string(&pokedex).expect("Bad");
            let file = std::fs::File::create("./pokedex/black-white.json").expect("File not created?");
            serde_json::to_writer_pretty(file, &json).expect("Something went wrong with serialisation");
        },
        None => println!("Something went wrong I guess")
    };

}
