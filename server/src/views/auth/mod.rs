mod login;
mod logout;

use crate::views::path::Path;

use actix_web::web;

pub fn factory(app: &mut web::ServiceConfig) {
    let base = Path::new("/auth");

    app.route(&base.sub("/login"), web::get().to(login::login) )
        .route(&base.sub("/logout"), web::get().to(logout::logout) );
}
