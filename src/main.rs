use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/home-info", web::get().to(get_home_info))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

async fn get_home_info() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", Home::create_predefine_home()))
}

#[derive(Debug)]
struct Home {
    power_status: bool,
    ip: String,
    name: String,
}

impl Display for Home {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Home: (power_status: {}; ip: {}; name: {})", self.power_status, self.ip, self.name)
    }
}

impl Home {
    fn create_predefine_home() -> Home {
        Home {
            power_status: true,
            ip: String::from("127.0.0.1"),
            name: String::from("sosnovka_home"),
        }
    }
}

