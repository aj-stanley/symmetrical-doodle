use ntex::web;
use std::sync::{Arc, Mutex};

struct AppState {
    app_name: String,
}

#[derive(Clone)]
struct AppStateWithCounter {
    counter: Arc<Mutex<i32>>,
}

async fn index(
    app_data: web::types::State<AppState>,
    counter_data: web::types::State<AppStateWithCounter>
) -> String {
    let mut counter = counter_data.counter.lock().unwrap();
    *counter += 1;
    
    format!("Request number: {counter} for app: {}", app_data.app_name)
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello!")
}
#[web::get("/st/{name}")]
async fn greet(data: web::types::State<AppState>, req: web::types::Path<String>) -> String {
    let name = req.into_inner();
    let app_name = &data.app_name;
    format!("Hello {app_name}!, {name}!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn hey(data: web::types::State<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hey {app_name}!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let counter = AppStateWithCounter {
        counter: Arc::new(Mutex::new(0)),
    };
    web::HttpServer::new(move || {
        web::App::new()
            .service(web::scope("/api").service(hello))
            .state(AppState {
                app_name: String::from("ntex"),
            })
            .service(greet)
            .service(echo)
            .route("/hey", web::get().to(hey))
            .state(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
