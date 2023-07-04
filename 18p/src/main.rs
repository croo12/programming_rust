use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct TestData {
    x: u64,
    y: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/test", web::post().to(test_action))
    });

    println!("Serving on http://localhost:3000 ...");

    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>test_homepage</title>
                <form action="/test" method="post">
                    <input type="text" name="x" />
                    <input type="text" name="y" />
                    <button type="submit">ACTION</button>
                </form>
            "#
        )
}

fn test_action(form : web::Form<TestData>) -> HttpResponse {
    if form.x == 0 || form.y == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Zero? Zero is suck");
    }

    let response = format!("this is format. x : {}, y : {}", form.x, form.y);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}