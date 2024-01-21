use pokedex_scraper::{scrape, Pokedex};

#[tokio::main]
async fn main() {
    // TODO Just note that you have to edit the name of the file yourself
    let url = "https://pokemondb.net/pokedex/game/black-white";

    // TODO This is a note that for now, locations have to be inputted manually, this is just because I haven't found a good way to do data scraping for this process
    /* let locations = vec!["Starter", "Twinleaf Town", "Route 201", "Lake Verity",
    "Route 202", "Route 203", "Oreburgh Gate", "Oreburgh City", "Oreburgh Mine",
    "Route 204", "Ravaged Path", "Floaroma Meadow",
    "Valley Windworks", "Route 205", "Eterna Forest", "Eterna City",
    "Old Chateau", "Route 206", "Wayward Cave", "Route 207",
    "Mt. Coronet", "Route 208", "Hearthome City", "Route 209",
    "Solaceon Ruins", "Lost Tower", "Route 210",
    "Route 215", "Veilstone City",
    "Route 214", "Maniac Tunnel", "Valor Lakefront", "Route 213", "Pastoria City", "Route 212",
    "Celestic Town", "Fuego Ironworks", "Route 219", "Route 220",
    "Route 221", "Route 218", "Canalave City", "Iron Island",
    "Route 211", "Route 216", "Route 217",
    "Acuity Lakefront", "Lake Acuity", "Distortion World",
    "Sendoff Spring", "Lake Valor", "Route 222", "Sunyshore City",
    "Route 223", "Pokémon League", "Victory Road"
    ]; */

    let locations = vec!["Starter", "Route 1", "Route 2", "Striaton City", "The Dreamyard",
    "Route 3", "Wellspring Cave", "Nacrene City", "Pinwheel Forest",
    "Route 4", "Desert Resort", "Relic Castle",
    "Route 5", "Driftveil City", "Cold Storage",
    "Route 6", "Chargestone Cave", "Route 7", "Celestial Tower",
    "Route 17", "Route 18", "P2 Laboratory", "Mistralton Cave",
    "Twist Mountain", "Icirrus City", "Dragonspiral Tower",
    "Route 8", "Moor of Icirrus", "Route 9",
    "Route 10", "Victory Road"];

    match scrape(url).await {
        Some(pokemon) => {
            println!("{:?}", pokemon);

            let pokedex = Pokedex { locations, pokemon };
            let json = serde_json::to_string(&pokedex).expect("Bad");
            let file = std::fs::File::create("./pokedex/black-white.json").expect("File not created?");
            serde_json::to_writer_pretty(file, &json)
                .expect("Something went wrong with serialisation");
        }
        None => println!("Something went wrong I guess"),
    };
}
