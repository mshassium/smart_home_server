use std::env;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(|| {
        App::new()
            .route("/home-info", web::get().to(get_home_info))
    })
        .bind(("0.0.0.0", port))
        .expect("Can not bind to port 8000")
        .run()
        .await
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

