#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (ignore invalid UTF-8)
    if let Ok(input) = std::str::from_utf8(data) {
        // Fuzz the NCL parser with random input
        // It should never panic, only return errors
        let _ = ncc::parse_root(input);
    }
});
