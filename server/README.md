Kodiak's web server is based on [Actix](https://actix.rs). It was introduced in 0.2.0 and offers a REST API to manage items in Kodiak.

# Usage

All requests return a unit in JSON format. In general the unit's type has to be provided. However, there is also 
an endpoint for a generic unit. As of 23/10/2021 Kodiak supports the following Unit Types.

* Asset
* Task
* User

## API Base URL

For the time being the API has version 1. So, the base URL for all API endpoints is: http://localhost:4000/rest/api/v1.

## Examples

Get all Assets.

    GET http://localhost:4000/rest/api/v1/assets

Get a specific Asset.

    GET http://localhost:4000/rest/api/v1/assets/{key}

# All Endpoints

## Arbitrary Units
```
/rest/api/v{version}/units
/rest/api/v{version}/units/key
```

## Kodiak defined Units

```
/rest/api/v{version}/assets
/rest/api/v{version}/assets/key

/rest/api/v{version}/tasks
/rest/api/v{version}/tasks/key

/rest/api/v{version}/users
/rest/api/v{version}/users/key
```

# Versioning

Kodiak uses a dedicated version for its REST API which is completely independent of Kodiak's server version.
However, Semantic Versioning Specification [(SemVer)](https://semver.org/) is also applied to the REST API. 
The URI of the API endpoint exposes only the major version. Minor and patch versions are only relevant internally.

For the time being, the REST API has version 0, as there might be breaking changes alongside Kodiak's journey to 1.0 release.

```
/rest/api/v0
```

# Testing

Ideally tests cover the external interface of the Kodiak server completely. Blackbox tests are the way to go and are located in a dedicated folder. 
This approach also affects the Kodiak server code base as only a library can be tested. That's why this crate has a library and a binary.

1. The library hosts all the functionality.
2. The binary hosts ``fn main``.

# Resources

The following articles helped me a lot in understanding design principles of REST APIs.

* [REST API: Key Concepts, Best Practices, and Benefits by altexsoft](https://www.altexsoft.com/blog/rest-api-design/)
* [Four REST API Versioning Strategies by xmatters](https://www.xmatters.com/blog/blog-four-rest-api-versioning-strategies/)
* [REST API Best Practices – Comprehensive Handbook by Paridhi Wadhwani](https://www.bacancytechnology.com/blog/rest-api-best-practices)

The article from Luca guided the approach for covering Kodiak's external interface with blackbox tests.

* [How To Bootstrap A Rust Web API From Scratch by Luca Palmieri](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/)
