use web_sys::window;

pub async fn blog() {
    let win = window().unwrap();
    let bod = win.document().unwrap();
    let feed_element = bod.get_element_by_id("blog-feed").unwrap();
    let feed_element_en = bod.get_element_by_id("blog-feed-en").unwrap();
    let feed_en = reqwest::get("https://blog.lovirent.eu/en/index.xml")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let feed = reqwest::get("https://blog.lovirent.eu/index.xml")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = rss::Channel::read_from(&feed[..]).unwrap();
    let channel_en = rss::Channel::read_from(&feed_en[..]).unwrap();
    let cont_en = channel_en
        .items()
        .iter()
        .map(|item| {
            format!(
                "<li><a href='{}'>{}</a></li>",
                item.link().unwrap(),
                item.title().unwrap()
            )
        })
        .collect::<String>();
    let cont = channel
        .items()
        .iter()
        .map(|item| {
            format!(
                "<li><a href='{}'>{}</a></li>",
                item.link().unwrap(),
                item.title().unwrap()
            )
        })
        .collect::<String>();
    if cont_en.is_empty() {
        feed_element.set_text_content(Some("no posts yet"));
    } else {
        feed_element_en.set_inner_html(&cont_en);
    }
    if cont.is_empty() {
        feed_element.set_text_content(Some("no posts yet"));
    } else {
        feed_element.set_inner_html(&cont);
    }
}
