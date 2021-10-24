mod auth;
mod path;

pub use path::Path;

use actix_web::web;

pub fn factory(app: &mut web::ServiceConfig) {
    auth::factory(app);
}
