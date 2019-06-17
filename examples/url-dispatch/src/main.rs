mod cfg;
mod dhandler;
mod minfo;
mod norm;
mod norm2;
mod path;
mod path2;
mod pbuf;
mod pred;
mod pred2;
mod resource;
mod scope;
mod url_ext;
mod urls;

// <main>
use actix_web::{web, App, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .route("/user/{name}", web::get().to(index))
        .route("/user/{name}", web::post().to(index));
}
// </main>
