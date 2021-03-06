/// 变量解构
/// 8.🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
/// > 提示: 可以使用变量遮蔽或可变性

// 修复下面代码的错误并尽可能少的修改
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

