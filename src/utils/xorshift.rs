use std::time::{SystemTime, UNIX_EPOCH};

pub fn xorshift(count: u8) {
    let time = SystemTime::now();
    let since_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut state: u32 = (since_epoch.as_micros() % u32::MAX as u128) as u32;

    for _ in 0..count {
        let rnd = xorshift_generate(&mut state);
        println!("{}", rnd)
    }
}

fn xorshift_generate(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    return x;
}
