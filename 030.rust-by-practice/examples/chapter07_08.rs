///8 🌟🌟 continue 会结束当次循环并立即开始下一次循环


// 填空，不要修改其它代码
fn main() {
    let mut n = 0;
    for _i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       break;
    }

    assert_eq!(n, 66);
}

