use reqwest::header::CONTENT_TYPE;
use kodiak_core::unit::{Unit, UnitType};

pub async fn create(client: reqwest::Client, unittype: UnitType, name: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets?name=", name),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks?name=", name),
        UnitType::User => format!("{}{}{}", baseurl, "/users?name=", name),
    };

    // Todo: error handling
    let unit = client.post(url).send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn read(client: reqwest::Client, unittype: UnitType, key: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets?key=", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks?key=", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users?key=", key),
    };

    // Todo: error handling
    let unit = client.get(url).send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn update(client: reqwest::Client, unittype: UnitType, key: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets?key=", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks?key=", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users?key=", key),
    };

    // Todo: error handling
    let unit = client
        .put(url)
        .header(CONTENT_TYPE, "application/json")
        .body(r#"{"Task":{"status":null,"subject":"TestME","unit":{"created":"2021-10-16T08:48:23.343296569Z","key":"fc0fa3fd-f93e-4026-b75e-24e69c6c0da9","state":"On","updated":null}}}"#)
        .send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}

pub async fn delete(client: reqwest::Client, unittype: UnitType, key: &str) -> Option<Unit> {
    let baseurl = "http://localhost:4000/rest/api/v1";

    let url = match unittype {
        UnitType::Asset => format!("{}{}{}", baseurl, "/assets?key=", key),
        UnitType::Task => format!("{}{}{}", baseurl, "/tasks?key=", key),
        UnitType::User => format!("{}{}{}", baseurl, "/users?key=", key),
    };

    // Todo: error handling
    let unit = client
        .delete(url)
        .send().await.ok()?.json::<Unit>().await.ok()?;
    Some(unit)
}
