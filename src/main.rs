use ntex::web;

struct AppState {
    app_name: String,
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello!")
}
#[web::get("/st/{name}")]
async fn hello_name(data: web::types::State<AppState>, req: web::types::Path<String>) -> String {
    let name = req.into_inner();
    let app_name = &data.app_name;
    format!("Hello {app_name}!, {name}!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn hey() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(web::scope("/api").service(hello))
            .state(AppState {
                app_name: String::from("Ntex"),
            })
            .service(hello_name)
            .service(echo)
            .route("/hey", web::get().to(hey))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
