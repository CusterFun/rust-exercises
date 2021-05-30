struct Custom {
    data: String,
}

impl Drop for Custom {
    // 为结构体 Custom 实现 Drop trait，就必须实现 drop 方法
    fn drop(&mut self) {
        println!("Dropping Custom with data: {}", self.data);
    }
}

fn main() {
    let str1 = Custom {
        data: String::from("hello world!"),
    };
    let str2 = Custom {
        data: String::from("hello rust!"),
    };
    println!("Custom created");
    println!("str1: {}", str1.data);
    println!("str2: {}", str2.data);
}
