/// Core database components:
/// - Data models (structs)
/// - Interfaces (Mapping structs to DB tables)
/// - Queries (Bulk extraction of DB data)
pub mod database;

/// Idiomatic methods for creating objects
pub mod import;

/// Methods for generating system configs from Themdex themes
pub mod apply;

/// Config struct to store and read user-configurable information
pub mod config;

/// Basic shared implementation details
/// - Custom error type (ThemedexError)
pub mod common;
