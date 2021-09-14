use actix_web::web;
use crate::views::Path;

use serde::{Deserialize};

mod assets;
mod tasks;
mod users;

#[derive(Deserialize)]
pub struct Key {
    key: String,
}

pub fn factory(app: &mut web::ServiceConfig) {
    let base = Path::new("/rest/api");

    app
        .route(&base.sub("/assets"), web::post().to(assets::create) )
        .route(&base.sub("/assets"), web::get().to(assets::read) )
        .route(&base.sub("/assets"), web::put().to(assets::update) )
        .route(&base.sub("/assets"), web::delete().to(assets::delete) );
}
