use std::{sync::Mutex};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod markov_chain;
mod dataset;
use markov_chain::MarkovChain;

struct AppState {
    markov_chain: Mutex<MarkovChain>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host_and_port = std::env::args().nth(1).unwrap_or(String::from("127.0.0.1:8080"));

    println!("Server launched at: {host_and_port}");
    
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {
            markov_chain: Mutex::new(MarkovChain::new())
        }))
        .service(web::scope("/api/markov_chain")
                    .route("", web::get().to(generate_sentence))
                    .route("", web::post().to(add_sentence))
        )
    })
    .bind(host_and_port)?
    .run()
    .await
}

async fn generate_sentence(state: web::Data<AppState>) -> impl Responder {
    let sentence = (*state.markov_chain.lock().unwrap()).generate(25);
    HttpResponse::Ok().body(sentence)
}

async fn add_sentence(request_body: String, state: web::Data<AppState>) -> impl Responder {
    (*state.markov_chain.lock().unwrap()).append(&request_body);
    HttpResponse::Ok().body(request_body)
}