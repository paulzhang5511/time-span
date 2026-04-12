# time-span

一个基于 RAII 的简单 Rust 时间测量库。

## 功能特性

- **基于 RAII 的时间测量**：当守卫离开作用域时自动记录耗时
- **便捷的宏**：避免手动声明变量
- **闭包支持**：测量闭包的耗时并返回其结果
- **轻量级**：仅依赖 `log` 库

## 使用方法

将以下内容添加到你的 `Cargo.toml`：

```toml
[dependencies]
time-span = "0.1"
log = "0.4"
```

### 基础 RAII 守卫

```rust
use time_span::TimeSpan;

fn main() {
    // 初始化日志记录器（例如 env_logger）
    env_logger::init();
    
    {
        let _guard = TimeSpan::new("heavy_work");
        // 执行一些耗时操作...
    } // 守卫被丢弃，时间被记录
}
```

### 使用宏

```rust
use time_span::time_span;

fn process() {
    time_span!("process_func");
    // 你的代码...
}
```

### 测量闭包

```rust
use time_span::measure_time;

let result = measure_time("calculation", || {
    // 一些昂贵的计算
    42
});

assert_eq!(result, 42);
```

## 许可证

本项目采用以下任一许可证授权：

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)

你可以自行选择。

## 贡献

除非你明确声明，否则你有意提交以供纳入本作品的任何贡献（如 MIT license 许可证所定义），应按照上述双重许可证进行授权，没有任何额外的条款或条件。
