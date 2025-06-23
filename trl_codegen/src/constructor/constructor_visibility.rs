//! # constructor_visibility
//! This module contains the `ConstructorVisibility` enum which represents visibility modifier of a constructor
//!
/// Enum `ConstructorVisibility` represents visibility modifier of a constructor
pub enum ConstructorVisibility {
    Pub,
    PubPath(String),
    Private,
}
