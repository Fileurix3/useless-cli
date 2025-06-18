use crate::utils::xorshift;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand_password(length: u8, count: u8) -> Result<(), Box<dyn std::error::Error>> {
    let time = SystemTime::now();
    let since_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut xorshift_state: u32 = (since_epoch.as_micros() % u32::MAX as u128) as u32;

    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";

    for _ in 0..count {
        let mut password = String::new();

        for _ in 0..length {
            let xorshift = xorshift::xorshift_generate(&mut xorshift_state) as usize;
            let rand_index: usize = xorshift % charset.len();
            let char = charset[rand_index] as char;
            password.push(char);
        }

        println!("{}", password);
    }

    Ok(())
}
