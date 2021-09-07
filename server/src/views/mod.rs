mod auth;
mod path;

use actix_web::web;

pub fn factory(app: &mut web::ServiceConfig) {
    auth::factory(app);
}
