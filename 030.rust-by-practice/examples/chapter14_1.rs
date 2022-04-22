// 1. cargo new hello-package
// 2. cargo new --lib hello-package
// 3. /* 使用你的答案填空 */

// Q: package 1# 和 2# 的区别是什么 ?
// A: 1# 是二进制包, 入口是 src/main.rs，2# 是库文件包，入口是 src/lib.rs
//
//
// 4. /* 填空 */

// Q: package `hello-package1` 中的库包名称是?
// A: hello-package1
//
// 5. 🌟🌟 为 hello-package 添加一个库包，并且完成以下目录结构的填空:
// # 填空
// .
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   ├── lib.rs
// │   └── main.rs
//
//
// 6. 🌟🌟🌟 一个 package 最多只能包含一个库包，
// 但是却可以包含多个二进制包：
// 通过将二进制文件放入到 src/bin 目录下实现: 
// 该目录下的每个文件都是一个独立的二进制包，包名与文件名相同，
// 不再与 package 的名称相同。.
//
/**
# 创建一个 a package 包含以下包： 
# 1. 三个二进制包: `hello-package`, `main1` and `main2`
# 2. 一个库包
# 并完成以下目录结构的填空
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # 存放集成测试文件的目录
│   └── some_integration_tests.rs
├── benches # 存放 benchmark 文件的目录dir for benchmark files
│   └── simple_bench.rs
└── examples # 存放示例文件的目录
    └── simple_example.rs

* /

