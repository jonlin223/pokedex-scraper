use scraper::{Html, Selector};
use serde::Serialize;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize)]
pub struct Pokemon {
    name: String,
    types: Vec<String>,
    sprite: String
}

pub async fn scrape(url: &str) -> Option<Vec<Pokemon>> {

    let res = reqwest::get(url).await.ok()?.text().await.ok()?;
    let doc = Html::parse_document(&res);
    let selector = Selector::parse("div.infocard").ok()?;

    let name_selector = Selector::parse("span.infocard-lg-data>a").ok()?;
    let type_selector = Selector::parse("span.infocard-lg-data>small>a").ok()?;
    let image_selector = Selector::parse("span.infocard-lg-img>a>img").ok()?;

    let client = reqwest::Client::new();
    let mut pokedex = vec![];

    for entry in doc.select(&selector).map(|x| x.html()) {
        let entry_doc = Html::parse_document(&entry);

        let pokemon = entry_doc.select(&name_selector).map(|x| x.inner_html()).next()?;
        let types = entry_doc.select(&type_selector).map(|x| x.inner_html()).collect::<Vec<_>>();
        let img = entry_doc.select(&image_selector).map(|x| x.value().attr("src")).next()??;

        let img_info = img.split("https://img.pokemondb.net/sprites/").nth(1)?;
        let img_name = String::from(img_info.split("/").nth(0)?) + "_" + img_info.split("/").last()?;
        let path = String::from("./pokedex/sprites/") + &img_name;

        let img_bytes = client.get(img).send().await.ok()?.bytes().await.ok()?;
        let mut file = tokio::fs::File::create(path).await.ok()?;
        file.write_all(&img_bytes).await.ok()?;

        // println!("{} - {:?} - {}", pokemon, types, img_name);
        pokedex.push(Pokemon { name: pokemon, types: types, sprite: img_name });
    }

    Some(pokedex)
}