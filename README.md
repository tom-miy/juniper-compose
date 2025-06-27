# juniper_compose_ng

Merge multiple [Juniper](https://docs.rs/juniper) object definitions into a single object type.

[crates.io](https://crates.io/crates/juniper_compose_ng) | [docs](https://docs.rs/juniper_compose_ng) | [github](https://github.com/tom-miy/juniper-compose)

## Juniper 0.16 Compatibility

This is a fork of the original juniper-compose crate that has been updated to support Juniper 0.16. The original crate was designed for earlier versions of Juniper, and this fork includes the necessary modifications to ensure compatibility with Juniper 0.16's API changes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
juniper_compose_ng = "0.16.2"
```

## Motivation

You are building a GraphQL server using Juniper. At some point you realize that you have gigantic
Query and Mutation types:

```rust
#[derive(Default)]
struct Query;

#[juniper::graphql_object]
impl Query {
    async fn user(ctx: &Context, id: Uuid) -> User {
        // ...
    }

    async fn users(ctx: &Context) -> Vec<User> {
        // ...
    }

    async fn task(ctx: &Context, id: Uuid) -> Task {
        // ...
    }

    async fn tasks(ctx: &Context) -> Vec<Task> {
        // ...
    }
    
    // ...many more
}
```

You would like to split it up into multiple domain-specific files, and have e.g. all User
queries in one file and all Task queries in the other. With current Juniper API, it is very
hard to do, but this crate can help you.

## Usage

```rust
#[derive(Default)]
struct UserQueries;

#[composable_object]
#[juniper::graphql_object]
impl UserQueries {
    async fn user(ctx: &Context, id: Uuid) -> User {
        // ...
    }

    async fn users(ctx: &Context) -> Vec<User> {
        // ...
    }
}

#[derive(Default)]
struct TaskQueries;

#[composable_object]
#[juniper::graphql_object]
impl TaskQueries {
    async fn task(ctx: &Context, id: Uuid) -> Task {
        // ...
    }

    async fn tasks(ctx: &Context) -> Vec<Task> {
        // ...
    }
}

composite_object!(Query(UserQueries, TaskQueries));
```

Custom contexts are supported:

```rust
composite_object!(Query<Context = MyCustomContext>(UserQueries, TaskQueries));
```

Visibility specifier for generated type is supported:

```rust
composite_object!(pub(crate) Query<Context = MyCustomContext>(UserQueries, TaskQueries));
```

Custom scalars are currently not supported, but will be added if requested.

## Credits

This crate is a fork of the original [juniper-compose](https://github.com/nikis05/juniper-compose) by Kit Isaev, updated to support Juniper 0.16. We thank the original author for their excellent work.
