//! # vitals-cli
//!
//! A local-first health tracking CLI library.
//!
//! This crate provides the core domain logic, storage, and CLI plumbing
//! for tracking health metrics such as blood pressure, physical activity,
//! and nutrition. It is designed so that new metrics can be added without
//! major refactors.
//!
//! This library never performs medical diagnosis; it only stores and
//! summarizes user-provided data.
