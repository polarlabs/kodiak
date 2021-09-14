Kodiak is a tool to manage any arbitrary item, such as assets, task and users. It is implemented in Rust. The project is in its early stages with a roadmap to version 1.0 still work in progress.

# Version 0.1.0

Version 0.1.0 is a Minimum Viable Product with a focus on data structures. It leverages trait objects to implement a generic interface which keeps the code base flexible for future development.

## Release Notes

1. Manage (create, read, update, delete) Assets, Tasks and Users.
2. State is represented internally as a HashMap provided by [Rust Standard Library](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
3. Persist state to disk via file in JSON format compatible with [Serde](https://serde.rs/).
4. User interface: CLI only, using sub commands with specific options / parameters based on clap.

## Known Issues

1. Test coverage: not existent.
2. Other todos are documented with ```// todo:``` in the code base.
3. License terms: tbd.

# Roadmap and future Considerations

Please be aware that the following roadmap is still Work in Progress and might be changed at any time.

For release 0.5.0:

* Allow definition of custom object types
* Custom object types are based on Kodiak's units but also allow to add further attributes.

For release 0.4.0:

* Implement Data Persistence layer
* Supported databases: tbd

For release 0.3.0:

* Implement a simple Web Frontend delivered by Kodiak's web server
* Refine data structures to leverage best practices from Domain Driven Design and Functional Programming 
* Evaluate usage of [Refinement](https://crates.io/crates/refinement) in Frontend and Backend
 
For release 0.2.0:

* Implement a Web Server based on [ACTIX](https://actix.rs/) to manage items via REST API
* Update CLI to use REST API

# Version Numbering Scheme

In principal, Kodiak follows the Semantic Versioning Specification [(SemVer)](https://semver.org/). However, for the time being Kodiak does not meet all requirements of the specification, e.g. Kodiak 0.1.0 has no public API. Declaration of a public API is planned for 0.2.0.

As long as version numbers are < 1.0, Kodiak's development might be jumpy including the data persistence layer. Each 0.x release will represent a major developement step towards a fully functional web application, including data, application and presentation layer.

# Tools

To create Kodiak, the following tools are in use:

* [Rust Programming Language](https://www.rust-lang.org/)
* [Crates from Rust Ecosystem](https://crates.io/): [clap](https://crates.io/crates/clap), [chrono](https://crates.io/crates/chrono), [serde](https://crates.io/crates/serde), [typetag](https://crates.io/crates/typetag) and [uuid](https://crates.io/crates/uuid)
* [IntelliJ Idea Community Edition](https://www.jetbrains.com/idea/)
* [Apostrophe](https://gitlab.gnome.org/World/apostrophe) for Markdown editing
* [Github](https://github.com/) and [Github CLI](https://github.com/cli/cli)
* [Manjaro Linux](https://manjaro.org/)

# References

I've found some very good articles regarding Traits and Trait objects.

* [Rust traits: A deep dive by LogRocket](https://blog.logrocket.com/rust-traits-a-deep-dive/)
* [dyn Trait and impl Trait in Rust by Nick Cameron](https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust/)

Meanwhile I've discovered a great talk on leveraging Domain Driven Design and Functional Programming.

* [Talk Session: Domain Modeling Made Functional by Scott Wlaschin](https://www.youtube.com/watch?v=PLFl95c-IiU)
