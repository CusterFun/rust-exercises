/// 操作 UTF-8 字符串
/// 12. 🌟


fn main() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

/// utf8_slice
/// 我们可以使用三方库 utf8_slice 来访问 UTF-8 字符串的某个子串，但是与之前不同的是，该库索引的是字符，而不是字节.

/// Example
use utf8_slice;
fn utf8_slice_fn() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // 结果是 "🚀"
}
