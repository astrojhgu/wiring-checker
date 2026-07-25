# Wiring

**Wiring** 是一个使用 **Rust** 编写的强类型硬件连接描述库。

它的目标不是绘制原理图，也不是替代 KiCad、Altium 等 EDA 工具，而是用于**描述硬件系统中的连接关系，在编译期检查各种接口是否能够正确连接，并自动生成 D2 系统框图。**

本项目最初来源于射频和射电天文实验中的一个实际需求。

每次外出做野外实验之前，都需要反复检查：

- 天线是否接对？
- 电缆是否带齐？
- 公母接头是否匹配？
- 是否遗漏了某个转接器？
- 光模块、光纤、电源线是否能够正确连接？

这些问题如果到了实验现场才发现，往往意味着浪费大量时间，甚至导致整次实验失败。

Wiring 希望把这些问题提前到**编译阶段**。

如果系统能够成功编译，就意味着所有连接关系在机械接口层面都是合法的。

---

# 特性

- 使用 Rust 描述整个硬件系统
- 编译期检查连接器兼容性
- 强类型 Connector 系统
- 自动生成 D2 系统框图
- 易于扩展新的 Component
- 易于扩展新的 Connector
- API 简洁直观

---

# 示例

```rust
let mut ctx = D2Context::new(File::create("system.d2")?);

new_component!(ant, Ant, &mut ctx, "天线");
new_component!(cable, NMale2NMaleCable, &mut ctx, "N-N 同轴");
new_component!(lna, Lna, &mut ctx, "LNA");

connect!(ant.rf_out, cable.p1, &mut ctx);
connect!(cable.p2, lna.rf_in, &mut ctx);
```

生成：

```d2
天线: {
    ...
}

N-N 同轴: {
    ...
}

LNA: {
    ...
}

天线.rf_out -> N-N 同轴.p1
N-N 同轴.p2 -> LNA.rf_in
```

---

# 工作原理

Wiring 将 Rust 作为硬件系统描述语言。

用户负责实例化各种硬件模块，并描述它们之间的连接关系。

```
Rust 程序
    │
    ▼
编译期类型检查
    │
    ▼
生成 D2
    │
    ▼
SVG / PDF 系统框图
```

D2 只是最终的渲染输出，而不是数据源。

整个程序并不维护复杂的内部连接数据库，而是在实例化组件和建立连接的同时直接生成对应的 D2 描述。

---

# Connector

每一种连接器都对应一个 Rust 类型，例如：

```rust
Port<SmaMale>
Port<SmaFemale>

Port<NMale>
Port<NFemale>

Port<QsfpPlug>
Port<QsfpJack>
```

连接兼容关系由 Trait 描述：

```rust
impl Compatible<SmaMale, SmaFemale> for () {}
impl Compatible<SmaFemale, SmaMale> for () {}
```

因此：

```rust
connect!(sma_male, sma_female, &mut ctx);
```

能够通过编译，而：

```rust
connect!(sma_male, n_female, &mut ctx);
```

将在编译阶段直接报错，而无需任何运行时检查。

---

# 为什么使用 Rust？

Rust 的类型系统非常适合表达硬件接口之间的兼容关系。

Connector 类型就是 Rust 类型。

连接是否合法由编译器进行验证。

因此，编译器本身就是一个硬件连接检查器。

---

# 当前支持

目前主要面向射频实验系统，已经包含常见组件，例如：

- 天线
- 同轴电缆
- 转接头
- Bias-T
- LNA
- 滤波器
- 电源
- 电源分配器
- 时钟源
- 时钟分配器
- SFP/QSFP 光模块
- 光纤
- FPGA 采集设备

新的 Component 和 Connector 可以非常方便地扩展。

---

# 未来计划

除了机械连接检查之外，未来还计划加入更多类型检查，例如：

- 频率范围
- 阻抗
- 电压等级
- 数据速率
- 信号方向
- 模拟/数字接口属性

并支持自动生成：

- 接线表
- BOM
- 系统文档
- 测试报告

---

# 项目状态

目前项目仍处于早期开发阶段。

欢迎提交 Issue、Pull Request，或贡献新的 Component、Connector 及改进建议。
