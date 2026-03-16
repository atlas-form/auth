pub mod entity;
pub mod table;

// Re-export db_core for convenience
pub use db_core;
// Re-export core utilities from db_core
pub use db_core::{
    BaseRepository, Error as SdkError, PaginatedResponse, PaginationParams, Repository, Result,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Test that db_core is accessible
        let _config = db_core::DatabaseConfig::new("test", "postgres://localhost/test");
    }
}

