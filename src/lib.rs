use scraper::{Html, Selector};

pub async fn scrape(url: &str) -> Option<()> {

    let res = reqwest::get(url).await.ok()?.text().await.ok()?;
    let doc = Html::parse_document(&res);
    let selector = Selector::parse("div.infocard").ok()?;

    for entry in doc.select(&selector).map(|x| x.html()) {
        let entry_doc = Html::parse_document(&entry);
        let name_selector = Selector::parse("span.infocard-lg-data>a").ok()?;
        let type_selector = Selector::parse("span.infocard-lg-data>small>a").ok()?;
        let image_selector = Selector::parse("span.infocard-lg-img>a>img").ok()?;

        let pokemon = entry_doc.select(&name_selector).map(|x| x.inner_html()).next()?;
        let types = entry_doc.select(&type_selector).map(|x| x.inner_html()).collect::<Vec<_>>();
        let img = entry_doc.select(&image_selector).map(|x| x.value().attr("src")).next()??;

        println!("{} - {:?} - {}", pokemon, types, img);
    }
    Some(())
}