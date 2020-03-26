//! Tools for generating ad hoc values

use rand::RngCore;

/// Generate a n length base64 string (truncated to a factor of 4)
#[must_use]
pub fn generate_base64(chars: usize) -> String {
    // Calculate bytes needed for the chars
    let bytes = (chars / 4) * 3;

    // Create a buffer
    let mut data = vec![0_u8; bytes];
    // Fill the buffer
    rand::thread_rng().fill_bytes(&mut data);

    // Encode the string
    base64::encode_config(data, base64::URL_SAFE)
}
