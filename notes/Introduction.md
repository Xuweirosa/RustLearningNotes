# Introduction

#### Task 1 Start with "Hello world!"
```rust
fn main() {
    println!("Hello, world!");
}
```

- We can run the code in CMD for Windows.
```
> rustc main.rs
> .\main.exe
Hello, world!
```
- We can run the code in MacOS or Linux.
```
$ rustc main.rs
$ ./main
Hello, world!
```

- Notice:
    - 可执行的rust代码中，最先执行main函数
    - rust的缩进风格是四个空格，而不是一个制表符
    - println!是调用了宏，而不是调用了函数，如果调用函数，应该是println
    - 以分号结尾
- 编译和运行互相独立
    - 其中
    ```
    rustc main.rs
    ```
    是使用rust编译器编译它，编译成功后会生成二进制可执行文件。
    - 编译后一共有三个文件，包括$.rs$文件，$.exe$文件（可执行文件）和$.pdb$文件（包含调试信息）

#### Task 2 Use Cargo
Cargo是Rust的构建系统和包管理器，可以用来构建代码，下载依赖库并编译这些库。
- 使用Cargo创建新项目
```
$ cargo new hello_cargo ##新建了名为hello_cargo的项目
$ cd hello_cargo
```
- 此时hello_cargo目录下存在文件：
    - Cargo.toml
    - src目录
    - 在目录下初始化了一个git仓库
    - .gitignore文件

- Cargo期望源文件存放在src目录下，项目根目录只包括README, LICENSE, 配置文件，以及其他和代码无关的文件

- 构建并运行Cargo项目
    - 在hello_cargo目录下输入以下命令来构建项目
    ```
    cargo build
    ```
    - 该命令会创建一个可执行文件target\debug\hello_cargo.exe
    - 可以通过以下命令来运行该可执行文件
    ```
    .\target\debug\hello_cargo.exe
    ```
    - 也可以直接通过以下命令同时编译并运行生成的可执行文件
    ```
    cargo run
    ```
    - 可以通过以下命令快速检查代码确保其可以编译，但并不产生可执行文件（运行速度快，可以在编写时定期使用来检查已写代码）
    ```
    cargo check
    ```
- 发布（release）构建
    - 使用以下代码可以优化编译项目，并生成可执行文件
    ```
    cargo build --release
    ```
    - 这一优化可以使Rust代码运行得更快，但是编译用时更长
    - 适用于创建提供给用户使用的可执行文件，而不是程序员开发时使用的可执行文件
    - 生成的可执行文件存放于target\release中
    - 测试代码运行时间时，记得使用这一种可执行文件

- 使用git来处理已存在的项目代码
    ```
    $ git clone someurl.com/someproject
    $ cd someproject
    $ cargo build
    ```