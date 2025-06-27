#![warn(clippy::all)]
#![warn(clippy::pedantic)]

//! Merge multiple [Juniper](https://docs.rs/juniper) object definitions into a single object type.
//!
//! [crates.io](https://crates.io/crates/juniper-compose) | [docs](https://docs.rs/juniper-compose) | [github](https://github.com/nikis05/juniper-compose)
//!
//! ## Motivation
//!
//! You are building a GraphQL server using Juniper. At some point you realize that you have gigantic
//! Query and Mutation types:
//!
//! ```ignore
//! #[derive(Default)]
//! struct Query;
//!
//! #[juniper::graphql_object]
//! impl Query {
//!     async fn user(ctx: &Context, id: Uuid) -> User {
//!         // ...
//!     }
//!
//!     async fn users(ctx: &Context) -> Vec<User> {
//!         // ...
//!     }
//!
//!     async fn task(ctx: &Context, id: Uuid) -> Task {
//!         // ...
//!     }
//!
//!     async fn tasks(ctx: &Context) -> Vec<Task> {
//!         // ...
//!     }
//!     
//!     // ...many more
//! }
//! ```
//!
//! You would like to split it up into multiple domain-specific files, and have e.g. all User
//! queries in one file and all Task queries in the other. With current Juniper API, it is very
//! hard to do, but this crate can help you.
//!
//! ## Usage
//!
//! ```ignore
//! use juniper_compose_ng::{composable_object, composite_object};
//! use juniper::graphql_object;
//!
//! // Define your types and context
//! struct Context;
//! #[derive(juniper::GraphQLObject)] struct User { id: String }
//! #[derive(juniper::GraphQLObject)] struct Task { id: String }
//!
//! // Define composable query objects
//! #[derive(Default)]
//! struct UserQueries;
//!
//! #[composable_object]
//! #[graphql_object]
//! impl UserQueries {
//!     async fn user(&self, ctx: &Context, id: String) -> User {
//!         User { id }
//!     }
//! }
//!
//! #[derive(Default)]
//! struct TaskQueries;
//!
//! #[composable_object]
//! #[graphql_object]
//! impl TaskQueries {
//!     async fn task(&self, ctx: &Context, id: String) -> Task {
//!         Task { id }
//!     }
//! }
//!
//! // Compose them into a single Query type
//! composite_object!(Query(UserQueries, TaskQueries));
//! ```
//!
//! Custom contexts are supported:
//!
//! ```ignore
//! use juniper_compose_ng::composite_object;
//!
//! struct MyCustomContext;
//! #[derive(Default)] struct UserQueries;
//! #[derive(Default)] struct TaskQueries;
//!
//! composite_object!(Query<Context = MyCustomContext>(UserQueries, TaskQueries));
//! ```
//!
//! Visibility specifier for generated type is supported:
//!
//! ```ignore
//! use juniper_compose_ng::composite_object;
//!
//! struct MyCustomContext;
//! #[derive(Default)] struct UserQueries;
//! #[derive(Default)] struct TaskQueries;
//!
//! composite_object!(pub(crate) Query<Context = MyCustomContext>(UserQueries, TaskQueries));
//! ```
//!
//! Custom scalars are currently not supported, but will be added if requested.

use juniper::{GraphQLTypeAsync, Type};
use std::borrow::Cow;

/// Implements [ComposableObject](ComposableObject) for a GraphQL object type.
/// **Important**: must be applied before the `juniper::graphql_object` macro.
///
/// ## Example
///
/// ```ignore
/// use juniper_compose_ng::composable_object;
/// use juniper::graphql_object;
///
/// #[derive(Default)] struct UserQueries;
///
/// #[composable_object]
/// #[graphql_object]
/// impl UserQueries {
///     // ...
/// }
/// ```
pub use juniper_compose_macros_ng::composable_object;

/// Composes an object type from multiple [ComposableObject](ComposableObject)s.
/// Custom context type may be specified, otherwise defaults to `()`.
/// Custom visibility fro generated type may be specified.
///
/// ## Examples
///
/// ```ignore
/// use juniper_compose_ng::composite_object;
///
/// #[derive(Default)] struct UserQueries;
/// #[derive(Default)] struct TaskQueries;
/// #[derive(Default)] struct UserMutations;
/// #[derive(Default)] struct TaskMutations;
/// struct MyContextType;
///
/// composite_object!(Query(UserQueries, TaskQueries));
/// composite_object!(Mutation<Context = MyContextType>(UserMutations, TaskMutations));
/// composite_object!(pub QueryPublic(UserQueries, TaskQueries));
/// ```
pub use juniper_compose_macros_ng::composite_object;

/// Object types that you want to compose into one must implement this trait.
/// Use [composable_object](composable_object) to implement it.
pub trait ComposableObject: GraphQLTypeAsync + Default
where
    Self::Context: Sync,
    Self::TypeInfo: Sync,
{
    /// Returns a list of fields that exist on this object type.
    fn fields() -> &'static [&'static str];
}

#[doc(hidden)]
#[allow(clippy::must_use_candidate)]
pub fn type_to_owned(ty: &Type<'_>) -> Type<'static> {
    match ty {
        Type::Named(name) => Type::Named(Cow::Owned(name.to_string())),
        Type::NonNullNamed(name) => Type::NonNullNamed(Cow::Owned(name.to_string())),
        Type::List(inner, size) => Type::List(Box::new(type_to_owned(inner)), *size),
        Type::NonNullList(inner, size) => Type::NonNullList(Box::new(type_to_owned(inner)), *size),
    }
}
