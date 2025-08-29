use axum::response::Html;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index_page() -> Html<String> {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}
