//! Fuzzer for `Value` parsing.

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut slice = data;
    let result: Result<ciborium::value::Value, ciborium::de::Error<std::io::Error>> =
        ciborium::de::from_reader(&mut slice);
    if let Ok(value) = result {
        let mut reserialized = Vec::new();
        assert!(
            ciborium::ser::into_writer(&value, &mut reserialized).is_ok(),
            "A `Value` produced by deserialization should be serializable"
        );
        // Most of the time `reserialized` will equal `data`, but not always, so
        // we can't `assert_eq!(reserialized, data)`.
    }
});
