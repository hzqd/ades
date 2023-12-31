use crate::{Padding, aes_enc, aes_dec};
use aoko::{no_std::functions::ext::{AnyExt1, FnOnceExt}, l};
use std::fs;

pub fn aes(r#in: String, out: String, aes: String, encrypt: bool) {
    // Read file data & Write file as partially applied function:
    l!(
        data = fs::read(r#in).unwrap();
        write = |text| fs::write.partial2(text)(out).unwrap());

    // Initialize the remaining arguments:
    aes.padding(32).as_bytes().let_owned(|key| {
        // Crypto as partially applied function:
        l!(
            aes_enc = |data| aes_enc(key)(data);
            aes_dec = |data| aes_dec(key)(data));

        // Encryption and decryption:
        match encrypt {
            true => aes_enc(&data).let_owned(write),
            false => aes_dec(&data).let_owned(write)
        }
    })
}
