pub use auth::AuthClient;
pub use cluster::ClusterClient;
pub use election::ElectionClient;
pub use kv::KvClient;
pub use lease::LeaseClient;
pub use lock::{LockClient, Session, Xutex};
pub use maintenance::MaintenanceClient;
pub use watch::WatchClient;

/// Auth client.
mod auth;
/// Cluster client
mod cluster;
/// Election client.
mod election;
/// Kv client.
mod kv;
/// Lease client.
mod lease;
/// Lock client.
pub mod lock;
/// Maintenance client.
mod maintenance;
/// Watch client.
mod watch;

/// Default session ttl
pub const DEFAULT_SESSION_TTL: i64 = 60;
