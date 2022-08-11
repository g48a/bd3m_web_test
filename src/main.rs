use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_service::Service;

mod state;
mod processes;
mod views;
mod to_do;
mod json_serialization;

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}!")
}

#[allow(dead_code)]async fn logout() -> String {
    format!("Logout view")
}

#[allow(dead_code)]
async fn login() -> String {
    format!("Login view")
}

const PORT: i16 = 9980;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Worker is up and running.");
        let app = App::new()
            .wrap_fn(|req, srv| {
                //srv => routing
                //req => service request
                if *&req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error : {}", message)
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            }).configure(views::views_factory);
        return app
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
    })
        .bind(format!("0.0.0.0:{PORT}"))?
        .workers(10)
        .run()
        .await
}
