# 编译器时代码检查：端口/器件总数匹配

## 优化目标

将 `connect!` 中端口/器件总数的运行时 `assert_eq!` 改为**编译期** trait bound 检查。

## 原始问题

```rust
// src/lib.rs (改写前)
pub fn connect_to(...) {
    assert_eq!(M * N, P * Q, "...");  // 运行时检查
    ctx.add_edge(self_name, other_name);
}
```

`M` = 器件数量, `N` = 每器件的端口数。连接时要求 `M×N == P×Q`，但检查在运行时。

## 实现策略：将 TOTAL 编码为独立 const 泛型参数

核心难点：`generic_const_exprs` 在 where 子句中无法求值函数级 const 泛型表达式 `{ P * Q }`。  
解法：在 `Group` 中增加第四个 const 泛型参数 `TOTAL`，在 `component!` 宏展开时直接填入 `{ M * $n }` 的计算结果，使得 where 子句中只需比较纯 const 泛型参数而非表达式。

### 改动文件

| 文件 | 改动 |
|------|------|
| `src/lib.rs` | 1) 新增 `Eq<A, B>` + `EqCheck` trait（仅 `A==B` 时实现）；2) `Group` 增加 `TOTAL` 参数；3) `connect_to` 的 where 子句使用 `Eq<TOTAL, TOTAL_R>: EqCheck`；4) `component!` 宏在 struct/impl 上增加 `[(); { M * $n }]: Sized` 约束 |
| `src/main.rs` | 增加 `#![feature(generic_const_exprs)]` 和 `#![allow(incomplete_features)]`（宏展开需要） |

### 关键代码

```rust
// 编译期相等检查：Eq<A, B>: EqCheck 仅当 A == B
pub struct Eq<const A: usize, const B: usize>;
pub trait EqCheck {}
impl<const N: usize> EqCheck for Eq<N, N> {}

// Group 增加 TOTAL 参数
pub struct Group<T: Copy + Clone, const M: usize, const N: usize, const TOTAL: usize>;

// connect_to 的 where 子句
pub fn connect_to<..., const TOTAL_R: usize>(..., _other: Group<..., TOTAL_R>, ...)
where
    (): Compatible<C, B>,
    Eq<TOTAL, TOTAL_R>: EqCheck,  // ← 编译期检查
{
    ctx.add_edge(self_name, other_name);
}
```

### 宏展开示例

`component!(Lna { rf_in: (SmaFemale, 1) })` 展开为：

```rust
pub struct Lna<const M: usize>
where
    [(); { M * 1 }]: Sized,
{
    pub rf_in: Group<Connector<SmaFemale>, M, 1, { M * 1 }>,
    //                                      ^^^  ^^^  ^^^^^^^^
    //                                      器件数  每端口数  TOTAL
}
```

### 编译期错误示例

```rust
new_component!(ant, BiPolAnt, 16, "天线");    // 16×2 = 32
new_component!(cable, NMale2NMaleCable, 16);  // 16×1 = 16

connect!(ant.rf_out, cable.p1);
// error[E0308]: expected 32, found 16
//              ^^^ TOTAL    ^^^^ TOTAL_R
```

## 要求

- Rust nightly（`generic_const_exprs` 特征，当前验证于 rustc 1.100.0-nightly）
- 使用 `#![feature(generic_const_exprs)]` 和 `#![allow(incomplete_features)]`

## 未来的改进方向

- 当 `generic_const_exprs` 稳定后，可以移除 `TOTAL` 参数，直接在 where 子句中使用 `Eq<{ M * N }, { P * Q }>: EqCheck`
- 也可以改用 `const { assert!(M * N == P * Q); }`（当前 `inline_const` 不支持函数级 const 泛型参数）