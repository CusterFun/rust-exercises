/// å¸¸è§ç panic
/// 2. ðð

// ä¿®å¤ææç panicï¼è®©ä»£ç å·¥ä½
fn main() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(3);

    // å¤§é¨åæ¶åç¼è¯å¨æ¯å¯ä»¥å¸®æä»¬æååç°æº¢åºéè¯¯ï¼å¹¶é»æ­¢ç¼è¯éè¿ãä½æ¯ä¹æä¸äºæ¶åï¼è¿ç§æº¢åºé®é¢ç´å°è¿è¡ææä¼åºç°
    let v = production_rate_per_hour(2);

    divide(15, 1);

    println!("Success!")
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => speed as f64 * cph as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
