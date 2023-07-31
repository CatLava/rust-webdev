use warp::Filter;
use ch3::{Question, QuestionId};


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
