采用发布配置自定义构建:
    cargo build 采用的dev配置
        // 当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，Cargo 会对每一个配置都采用默认设置
        [profile.dev]
        opt-level = 0
    cargo build --release 采用的release配置
        // 设置控制 Rust 会对代码进行何种程度的优化。这个配置的值从 0 到 3。越高的优化级别需要更多的时间编译
        [profile.release]
        opt-level = 3
将crate发布到Crates.io
    文档注释
        /// Adds one to the number given. --> 支持markdown格式
        使用cargo doc生成这个文档注释的 HTML 文档
            --> 这个命令运行由 Rust 分发的工具 rustdoc 并将生成的 HTML 文档放入 target/doc 目录
            --> cargo doc --open 构建并打开
    常用（文档注释）部分
        # Examples  文档注释作为测试 --> cargo test会运行示例代码
        # Panics
        # Errors
        # Safety
    注释包含项的结构
        //! 开头的注释 整个crate的文档
    使用 pub use 导出合适的公有 API
        pub use self::kinds::PrimaryColor;
        pub use self::kinds::SecondaryColor;
        pub use self::utils::mix;
            -->  生成的 API 文档会在首页列出重导出的项以及其链接, 更易于查找
            -->  使用 use test_01::PrimaryColor;
    创建Crates.io账号
        在你可以发布任何 crate 之前，需要在 crates.io 上注册账号并获取一个 API token
        一旦登录之后，查看位于 https://crates.io/me/ 的账户设置页面并获取 API token
        登录: cargo login abcdefghijklmnopqrstuvwxyz012345
    发布新 crate 之前
        Cargo.toml
        [package]
        name = "guessing_game" # 唯一
        version = "0.1.0"
        edition = "2021"
        description = "A fun game where you guess what number the computer has chosen."
        license = "MIT OR Apache-2.0"
    发布:
        cargo publish
            --> 发布是永久性的, 对应版本不可能被覆盖, 其代码也不可能被删除
        cargo yank --vers 1.0.1
            --> 撤回版本, 阻止任何将来的项目将他们加入到依赖中
            --> cargo yank --vers 1.0.1 --undo 撤销撤回操作
Cargo工作空间:
    创建工作空间:
        工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包
        mkdir add --> cd add
            --> 创建Cargo.toml文件: 配置了整个工作空间
                [workspace]
                members = [
                    "adder",
                    "add_one",
                ]
                --> 在add目录下, cargo new adder
                    --> adder依赖add_one
                        adder/Cargo.toml
                            [dependencies]
                            add_one = { path = "../add_one" }
                        使用add_one函数
                            use add_one;
                            ...
                            add_one::add_one(num);
                --> 在add目录下, cargo new add_one --lib
                --> 在 add 目录中运行 cargo build 来构建工作空
                --> 为了在顶层 add 目录运行二进制 crate，可以通过 -p 参数和包名称来运行 cargo run 指定工作空间中我们希望使用的包
                    cargo run -p adder --> 这会运行 adder/src/main.rs 中的代码，其依赖 add_one crate
                    cargo test -p add_one --> 运行指定crate的测试
    在工作空间中依赖外部包
        add_one/Cargo.toml
            [dependencies]
            rand = "0.8.3"
// todo 使用 cargo install 从 Crates.io 安装二进制文件:
    cargo install 命令用于在本地安装和使用二进制 crate
    所有来自 cargo install 的二进制文件都安装到 Rust 安装根目录的 bin 文件夹中
// todo Cargo 自定义扩展命令