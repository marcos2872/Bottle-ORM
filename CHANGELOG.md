# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### UUID Support (Versions 1-7)
- **Full UUID Support**: Added comprehensive support for all UUID versions (1 through 7)
  - Version 1: Time-based with MAC address
  - Version 3: Name-based using MD5 hash
  - Version 4: Random (most common)
  - Version 5: Name-based using SHA-1 hash
  - Version 6: Reordered time-based (better for database indexing)
  - Version 7: Unix timestamp-based (sortable, recommended for new projects)
- Added `uuid` dependency with features for all versions: `v1`, `v3`, `v4`, `v5`, `v6`, `v7`, `serde`
- Updated type mapping in `types.rs` to handle `Uuid` → `UUID` SQL type
- Updated `query_builder.rs` to properly bind UUID values in INSERT operations
- Added UUID examples in README.md demonstrating usage with different versions

#### Documentation Improvements
- **Comprehensive Code Comments**: Added detailed documentation following Rust best practices
  - Module-level documentation for all files
  - Function-level documentation with examples and parameter descriptions
  - Inline comments explaining complex logic
  - Type and trait documentation with usage examples
  
- **Organized Structure**: Improved code organization
  - Clear section separators with comment blocks
  - Grouped related functionality
  - Consistent comment style across all modules

### Changed

#### Code Organization
- **lib.rs**: Complete reorganization with detailed module documentation
  - Added module-level documentation
  - Organized imports and re-exports with descriptive comments
  - Added quick start example in documentation

- **query_builder.rs**: Enhanced with comprehensive documentation
  - Detailed documentation for all public methods
  - Added examples for UUID filtering and querying
  - Documented filter types and query building process
  - Added type-safe binding documentation

- **database.rs**: Improved with detailed connection and schema management docs
  - Documented all driver types and their differences
  - Added comprehensive examples for connection strings
  - Documented table creation and foreign key management
  - Explained SQL dialect differences across drivers

- **migration.rs**: Enhanced migration documentation
  - Documented two-phase migration approach
  - Added examples for complex migration scenarios
  - Explained task execution order
  - Added idempotency documentation

- **model.rs**: Complete trait and structure documentation
  - Documented `Model` trait with examples
  - Added `ColumnInfo` field-by-field documentation
  - Included manual implementation examples
  - Added comprehensive attribute documentation

- **errors.rs**: Improved error handling documentation
  - Documented all error variants with use cases
  - Added error handling examples
  - Included helper methods for error creation
  - Added test examples

- **types.rs** (macro): Enhanced type mapping documentation
  - Documented all supported type mappings
  - Added examples for each type conversion
  - Explained Option<T> handling
  - Documented UUID support in detail

- **derive_model.rs** (macro): Improved macro implementation docs
  - Documented macro expansion process
  - Added attribute parsing documentation
  - Explained generated code structure
  - Added comprehensive examples

- **lib.rs** (macro): Complete macro crate documentation
  - Added overview of macro system
  - Documented all supported attributes
  - Included complete usage examples
  - Added type support documentation

#### Bug Fixes
- Fixed unused `mut` warnings in `query_builder.rs`
- Fixed unused `Result` warnings for `.add()` calls
- Converted doc comments to regular comments in match arms (following Rust conventions)
- Removed non-existent `bottle` member from workspace configuration

---

## [0.1.1] - Previous Release

### Initial Features
- Basic ORM functionality
- PostgreSQL, MySQL, and SQLite support
- Fluent query builder
- Automatic migrations
- Foreign key support
- Basic type mapping

[Unreleased]: https://github.com/Murilinho145SG/bottle-orm/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Murilinho145SG/bottle-orm/releases/tag/v0.1.1