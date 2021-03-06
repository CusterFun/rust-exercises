#![allow(unused)]
// 5ãðð

// `print_refs` æä¸¤ä¸ªå¼ç¨åæ°ï¼å®ä»¬ççå½å¨æ `'a` å `'b` è³å°å¾è·å½æ°æ´»å¾ä¸æ ·ä¹
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* è®©ä¸é¢çä»£ç å·¥ä½ */
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` æ´»å¾ä¸å¤ä¹does not live long enough
    let y: & i32 = &_x;

    // å¨å½æ°åä½¿ç¨ `'a` å°ä¼æ¥éï¼åå æ¯ `&_x` ççå½å¨ææ¾ç¶æ¯ `'a` è¦å°
    // ä½ ä¸è½å°ä¸ä¸ªå°ççå½å¨æå¼ºè½¬æå¤§ç
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    // è¿éï¼four å nice ççå½å¨æå¿é¡»è¦æ¯å½æ° print_refs é¿

    failed_borrow();
    // `failed_borrow`  æ²¡æä¼ å¥ä»»ä½å¼ç¨å»éå¶çå½å¨æ `'a`ï¼å æ­¤ï¼æ­¤æ¶ç `'a` çå½å¨ææ¯æ²¡æä»»ä½éå¶çï¼å®é»è®¤æ¯ `'static`
}
