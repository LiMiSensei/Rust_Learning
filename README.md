# Rust学习项目

这是一个用于学习Rust编程语言的基础项目，包含了多个Rust核心语法和功能的演示。

## 项目结构

```
RustProject/
├── src/
│   ├── main.rs              # 主入口文件
│   ├── _1printIn.rs         # 输入输出演示
│   ├── _2rand_demo.rs       # 随机数演示
│   ├── _3string_parse.rs    # 字符串解析
│   ├── _4data_type.rs       # 数据类型
│   ├── _5loop.rs            # 循环结构
│   ├── _6slice.rs           # 切片
│   ├── _7struct.rs          # 结构体
│   ├── _8enum.rs            # 枚举
│   └── _9path.rs            # 模块路径
├── Cargo.toml               # 项目配置
└── Cargo.lock
```

## 功能模块

### 1. 输入输出 (`_1printIn.rs`)
- 从控制台读取用户输入
- 处理输入合法性
- 输出回显用户输入

### 2. 随机数 (`_2rand_demo.rs`)
- 使用 `rand` 库生成随机数
- 比较随机数与固定值
- 演示 `match` 表达式

### 3. 字符串解析 (`_3string_parse.rs`)
- 将字符串转换为数字
- 使用 `parse()` 方法
- 错误处理

### 4. 数据类型 (`_4data_type.rs`)
- 整数类型 (u8/u16/u32/u64/u128/usize, i8/i16/i32/i64/i128/isize)
- 浮点类型 (f32/f64)
- 字符类型
- 元组
- 数组

### 5. 循环 (`_5loop.rs`)
- `loop` 无限循环
- `while` 条件循环
- `for` 范围循环

### 6. 切片 (`_6slice.rs`)
- 字符串切片
- 引用和范围操作

### 7. 结构体 (`_7struct.rs`)
- 结构体定义
- 关联函数和方法
- 构造函数 `new()`

### 8. 枚举 (`_8enum.rs`)
- 枚举定义
- 枚举方法
- `match` 模式匹配

### 9. 模块路径 (`_9path.rs`)
- 模块定义
- 路径访问
- `pub` 关键字

## 运行项目

```powershell
cargo run
```

在 `main.rs` 中可以通过注释/取消注释来运行不同的演示模块。

## 依赖

- `rand = "0.10.1"` - 随机数生成库

## Rust版本

- Edition: 2024
