use warp::Filter;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct QuestionId(String);

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}


async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId("1".to_string()),
        "first question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );
    Ok(warp::reply::json(
        &question
    ))
}
#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030))
        .await;
}
