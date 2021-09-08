Kodiak's web server is based on [Actix](https://actix.rs). It was introduced in 0.2.0 and offers a REST API to manage items in Kodiak.

# Endpoints (WIP)

```
/rest/api
/rest/api/v{version}/{unit_type}[/{key}]
/rest/doc
```

## Kodiak defined units
```
/rest/api/v{version}/assets
/rest/api/v{version}/assets/key

/rest/api/v{version}/tasks
/rest/api/v{version}/tasks/key

/rest/api/v{version}/users
/rest/api/v{version}/users/key
```

## Arbitrary units
```
/rest/api/v{version}/units
/rest/api/v{version}/units/key
```

# Versioning

Kodiak uses a dedicated version for its REST API which is completely independent from Kodiak's server version.
However, Semantic Versioning Specification [(SemVer)](https://semver.org/) is also applied to the REST API. Only the major version is exposed in the URI of the API endpoint. Minor and patch versions are relevant only internally.

For the time being the REST API has version 0, as their might be breaking changes alongside Kodiak's journey to 1.0 release.

```
/api/rest/0
```

# Resources

The following articles helped me a lot in understanding design principles of REST APIs.

* [REST API: Key Concepts, Best Practices, and Benefits by altexsoft](https://www.altexsoft.com/blog/rest-api-design/)
* [Four REST API Versioning Strategies by xmatters](https://www.xmatters.com/blog/blog-four-rest-api-versioning-strategies/)
