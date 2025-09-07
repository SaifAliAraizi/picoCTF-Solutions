use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer: Vec<u8>, borrowed_string: &mut String) {
    // Key for decryption
    let key = String::from("CSUCKS"); // FIX: Added missing semicolon

    // Editing our borrowed value
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decryption object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return; // FIX: Correctly returns early on error
    }
    let xrc = res.unwrap();

    // --- Unsafe operations demonstration ---
    // Even though Rust is memory safe by default, sometimes you need to break out of the safety net.
    // The following block shows how to work with raw pointers using unsafe operations.
    unsafe {
        // Decrypt the flag
        let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);

        // Creating a pointer to the decrypted data
        let decrypted_ptr = decrypted_buffer.as_ptr();
        let decrypted_len = decrypted_buffer.len();
        
        // UNSAFE: Dereferencing a raw pointer must be wrapped in an unsafe block.
        let decrypted_slice = std::slice::from_raw_parts(decrypted_ptr, decrypted_len);

        // Convert the decrypted slice to a UTF-8 string and append it
        borrowed_string.push_str(&String::from_utf8_lossy(decrypted_slice));
    }
    println!("{}", borrowed_string); // FIX: Correct println! syntax outputs our modified string
}

fn main() {
    // Encrypted flag values
    let hex_values = [
        "41", "30", "20", "63", "4a", "45", "54", "76", "12", "90", "7e", "53", "63", "e1", "01", "35",
        "7e", "59", "60", "f6", "03", "86", "7f", "56", "41", "29", "30", "6f", "08", "c3", "61", "f9", "35",
    ];

    // Convert hexadecimal strings to bytes
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    // Declare the variable as mutable to allow changes within the function
    let mut party_foul = String::from("Using memory unsafe languages is a: ");
    // Pass a mutable reference so that `decrypt` can modify the original string.
    decrypt(encrypted_buffer, &mut party_foul);
}
