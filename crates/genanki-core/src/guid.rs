//! GUID generation for notes
//!
//! This module provides functionality to generate globally unique identifiers
//! based on note field values using BLAKE3 hashing.

use crate::config::FIELD_SEPARATOR_STR;

/// Generates a GUID based on the provided fields.
///
/// This function combines the input fields using an ASCII unit separator (`\x1F`)
/// and computes a BLAKE3 hash of the combined string. The hash is then encoded
/// as a hexadecimal string to form the GUID.
///
/// # Arguments
///
/// * `fields` - A slice of strings used to generate the GUID
///
/// # Returns
///
/// A hexadecimal string representation of the BLAKE3 hash (64 hex characters)
///
/// # Example
///
/// ```
/// use genanki_core::guid_for;
///
/// let fields = vec![
///     "What is the capital of France?".to_string(),
///     "Paris".to_string(),
/// ];
/// let guid = guid_for(&fields);
/// assert_eq!(guid.len(), 64); // BLAKE3 produces 32 bytes = 64 hex chars
/// ```
pub fn guid_for(fields: &[String]) -> String {
    // Combine all fields into a single string using a separator to avoid ambiguity
    let combined = fields.join(FIELD_SEPARATOR_STR);

    // Calculate BLAKE3 hash (outputs 32 bytes)
    let hash = blake3::hash(combined.as_bytes());

    // Convert to hexadecimal string
    hex::encode(hash.as_bytes())
}

/// Generates a shorter GUID (16 bytes instead of 32)
///
/// This matches the official Anki GUID format more closely
pub fn guid_for_short(fields: &[String]) -> String {
    let combined = fields.join(FIELD_SEPARATOR_STR);
    let hash = blake3::hash(combined.as_bytes());
    hex::encode(&hash.as_bytes()[..16])
}

/// Validates that a GUID string is properly formatted
///
/// # Arguments
///
/// * `guid` - The GUID string to validate
///
/// # Returns
///
/// `true` if the GUID is valid, `false` otherwise
pub fn is_valid_guid(guid: &str) -> bool {
    // GUIDs should be 64 hex characters (full BLAKE3) or 32 hex characters (truncated)
    let len = guid.len();
    if len != 64 && len != 32 {
        return false;
    }

    // Check all characters are valid hex
    guid.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_for() {
        let fields = vec![
            "Question".to_string(),
            "Answer".to_string(),
        ];
        let guid = guid_for(&fields);
        assert_eq!(guid.len(), 64);
        assert!(is_valid_guid(&guid));
    }

    #[test]
    fn test_guid_for_short() {
        let fields = vec![
            "Question".to_string(),
            "Answer".to_string(),
        ];
        let guid = guid_for_short(&fields);
        assert_eq!(guid.len(), 32);
        assert!(is_valid_guid(&guid));
    }

    #[test]
    fn test_guid_deterministic() {
        let fields = vec![
            "Test".to_string(),
            "Fields".to_string(),
        ];
        let guid1 = guid_for(&fields);
        let guid2 = guid_for(&fields);
        assert_eq!(guid1, guid2);
    }

    #[test]
    fn test_guid_different_fields() {
        let fields1 = vec!["A".to_string(), "B".to_string()];
        let fields2 = vec!["C".to_string(), "D".to_string()];
        assert_ne!(guid_for(&fields1), guid_for(&fields2));
    }

    #[test]
    fn test_is_valid_guid() {
        assert!(is_valid_guid(&"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));
        assert!(is_valid_guid(&"0123456789abcdef0123456789abcdef"));
        assert!(!is_valid_guid(&"invalid"));
        assert!(!is_valid_guid(&"0123456789abcdef"));
    }
}
