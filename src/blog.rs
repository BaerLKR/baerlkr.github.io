use web_sys::window;

pub async fn blog() {
    let win = window().unwrap();
    let bod = win.document().unwrap();
    let feed_element = bod.get_element_by_id("blog-feed").unwrap();
    let feed = reqwest::get("https://blog.lovirent.eu/index.xml")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = rss::Channel::read_from(&feed[..]).unwrap();
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
    if cont.is_empty() {
        feed_element.set_text_content(Some("no posts yet"));
    } else {
        feed_element.set_inner_html(&cont);
    }
}
