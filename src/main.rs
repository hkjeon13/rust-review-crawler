use serde_json;
use tqdm_rs;


fn xexymix(){
    let page = "https://review4.cre.ma/xexymix.com/reviews?page=".to_string();

    let li_selector = scraper::Selector::parse(r#"li[id^="review_"]"#).unwrap();
    let review_selector = scraper::Selector::parse(r#"div[class="review_list_v2__review_lcontent"] > div"#).unwrap();
    let score_selector = scraper::Selector::parse(r#"div[class="review_list_v2__score_section"] > div > div > span[class="visually-hidden"]"#).unwrap();
    let product_selector = scraper::Selector::parse(r#"div[class^="review_list_v2__product_section"] > a > div > div[class="review_list_v2__info_container"] > div[class="review_list_v2__product_name"]"#).unwrap();
    let content_selector = scraper::Selector::parse(r#"div[class="review_list_v2__content_section"] > div > div > div > div > div"#).unwrap();
    let image_selector = scraper::Selector::parse(r#"div[class="review_list_v2__image_section"] > div > ul > li > a > div > img"#).unwrap();
    let user_selector = scraper::Selector::parse(r#"div[class="review_list_v2__review_rcontent"]"#).unwrap();
    let profile_selector = scraper::Selector::parse(r#"div[class="review_list_v2__options_section"] > div > div"#).unwrap();

    for k in 0..10000{
        let mut outputs = vec![];
        for i in tqdm_rs::Tqdm::new(k*2..(k+1)*2) {
            let new_page = page.clone() + (i+1).to_string().as_str();
            let response = reqwest::blocking::get(new_page).unwrap().text().unwrap();
            let document = scraper::Html::parse_document(&response);
            let reviews = document.select(&li_selector);
            for review in reviews {
                let mut content = review.select(&review_selector).next().unwrap();

                let score = content.select(&score_selector).next().unwrap().inner_html();
                let name = content.select(&product_selector).next().unwrap().inner_html();
                let text = content.select(&content_selector).next().unwrap().inner_html().trim().replace("<br>", "\n");
                let mut images = content.select(&image_selector);
                let mut images= images.map(|x|x.value().attr("src").unwrap().replace("portrait_", "")).collect::<Vec<_>>();
                let user = review.select(&user_selector).next().unwrap();
                let info = user.select(&profile_selector).map(
                    |x| x.select(&scraper::Selector::parse(r#"span"#).unwrap()).map(|x| x.inner_html()).collect::<Vec<_>>()
                ).collect::<Vec<_>>();
                let output = serde_json::json!({
                    "product": name,
                    "rating": score,
                    "review": text,
                    "meta": info,
                    "images": images
                });
                outputs.push(output);
            }
        }
        std::fs::write(
            "review2/review_".to_string()+(((k+1)*20)*2).to_string().as_str()+".json",
            serde_json::to_string_pretty(&outputs).unwrap(),
        ).unwrap();
        break;
    }

}

fn main() {
    xexymix();
}
