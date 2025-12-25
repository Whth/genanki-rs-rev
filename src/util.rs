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
/// A hexadecimal string representation of the BLAKE3 hash
pub fn guid_for(fields: &[String]) -> String {
    // Combine all fields into a single string (using a separator to avoid ambiguity)
    let combined = fields.join("\x1F"); // Using ASCII unit separator

    // Calculate BLAKE3 hash (outputs 32 bytes)
    let hash = blake3::hash(combined.as_bytes());

    // Convert to hexadecimal string (Anki-style GUID is typically 16-byte hex, but 32-byte is also OK)
    // If you want to match Anki official (16 bytes), you can truncate:
    // hash.as_bytes()[..16].to_hex()
    hex::encode(hash.as_bytes())
}
