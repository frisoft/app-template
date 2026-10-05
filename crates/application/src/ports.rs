//! Ports — the boundaries the application needs the outside world to implement.
//!
//! Ports are **owned by the inner layer** and implemented by adapters on the
//! outside. This is what inverts the dependency direction and keeps the
//! application independent of any concrete technology.
//!
//! Define one trait per capability your workflows need, for example:
//!
//! ```ignore
//! use async_trait::async_trait;
//!
//! #[async_trait]
//! pub trait OrderRepository: Send + Sync {
//!     async fn load(&self, id: &OrderId) -> Result<Order, RepositoryError>;
//! }
//!
//! #[async_trait]
//! pub trait UnitOfWork: Send + Sync {
//!     async fn commit(&self, state: &Order, events: &[DomainEvent])
//!         -> Result<(), RepositoryError>;
//! }
//! ```
//!
//! Rules:
//!
//! - Define ports **here**, never in `infrastructure`.
//! - Keep them minimal: only the operations an application workflow calls.
//! - Map technology errors (SQL, HTTP, ...) into a port error; never leak
//!   `sqlx::Error`, `reqwest::Error`, etc. into this layer.
//!
//! The snippet above is illustrative; add your own ports below.

// TODO: add your port traits here.
