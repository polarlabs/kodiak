use reqwest::header::CONTENT_TYPE;
use kodiak_core::unit::{Unit, UnitType};

pub async fn create(client: reqwest::Client, unittype: UnitType, name: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    // todo: convert UnitType to path centrally
    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets?name=", name),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks?subject=", name),
        UnitType::User => format!("{}{}{}", baseurl, "/users?email=", name),
        _ => { unreachable!() }
    };

    // Todo: error handling
    let unit = client.post(url).send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn read(client: reqwest::Client, unittype: UnitType) -> Vec<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    // todo: convert UnitType to path centrally
    let url = match unittype {
        UnitType::Unit => format!("{}{}", baseurl, "/units"),
        UnitType::Asset => format!("{}{}", baseurl, "/assets"),
        UnitType::Task => format!("{}{}", baseurl, "/tasks"),
        UnitType::User => format!("{}{}", baseurl, "/users"),
    };

    // Todo: error handling
    let unit = client.get(url).send().await.ok().unwrap().json::<Vec<Unit>>().await.ok().unwrap();
    unit
}

pub async fn read_by_key(client: reqwest::Client, unittype: UnitType, key: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    // todo: convert UnitType to path centrally
    let url = match unittype {
        UnitType::Unit => format!("{}{}{}", baseurl, "/units/", key),
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets/", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks/", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users/", key),
    };

    // Todo: error handling
    let unit = client.get(url).send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn update(client: reqwest::Client, unittype: UnitType, key: &str, payload: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    // todo: convert UnitType to path centrally
    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets/", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks/", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users/", key),
        _ => { unreachable!() }
    };

    // Todo: error handling
    let unit = client
        .put(url)
        .header(CONTENT_TYPE, "application/json")
        //.body(r#"{"Task":{"status":null,"subject":"TestME","unit":{"created":"2021-10-16T08:48:23.343296569Z","key":"fc0fa3fd-f93e-4026-b75e-24e69c6c0da9","state":"On","updated":null}}}"#)
        .body(payload.to_owned())
        .send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn delete(client: reqwest::Client, unittype: UnitType, key: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    // todo: convert UnitType to path centrally
    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets/", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks/", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users/", key),
        _ => { unreachable!() }
    };

    // Todo: error handling
    let unit = client
        .delete(url)
        .send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}
