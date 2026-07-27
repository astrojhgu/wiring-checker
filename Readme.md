# Wiring Checker

**Wiring Checker** 是一个使用 Rust 编写的强类型硬件系统描述工具。

它允许用户使用 Rust 描述实际硬件系统中的组件、接口和连接关系，并利用 Rust 类型系统在编译阶段检查连接是否合法，同时自动生成：

- 系统框图（D2）
- 物料清单（BOM）

它主要面向射频、通信、射电天文等复杂实验系统。

---

## 为什么需要 Wiring Checker？

在复杂实验系统中，一个完整链路通常包含大量硬件：

```
天线
 |
同轴电缆
 |
Bias-T
 |
转接头
 |
滤波器
 |
LNA
 |
数字采集设备
 |
光模块
 |
光纤
 |
计算系统
```

在实际搭建或者野外实验之前，经常需要确认：

- 接头是否匹配？
- 电缆是否正确？
- 是否遗漏转接器？
- 信号方向是否正确？
- 电源和信号是否混接？
- 所需物料数量是否足够？

传统方式通常依靠人工检查，容易遗漏。

Wiring Checker 希望将这些检查提前到程序设计阶段：

> 让编译器帮助检查硬件系统是否能够正确连接。

---

# 核心思想

Wiring Checker 将 Rust 程序作为硬件系统的唯一描述源。

```
Rust Hardware Description
          |
          v
 Compile-time Checking
          |
          v
 D2 Diagram Generation
          |
          v
 SVG / PDF System Diagram
```

D2 只负责最终绘图，而不是系统模型。

---

# 基本使用方式

一个硬件系统由三个部分组成：

- Component（组件）
- Port（接口）
- Connection（连接）

例如：

```rust
component!(Pwr5V {
    ac_in: (ScrewTerminal, 1),
    dc_out: (ScrewTerminal, 1)
});
```

定义一个 5V 电源：

```
Pwr5V

 AC input
    |
 DC output
```

---

创建组件：

```rust
new_component!(
    ant,
    BiPolAnt,
    16,
    "天线",
    &mut ctx
);

new_component!(
    lna,
    Lna,
    32,
    "LNA",
    &mut ctx
);
```

其中：

- `ant`：Rust 变量名
- `BiPolAnt`：组件类型
- `16`：数量
- `"天线"`：描述信息

---

连接组件：

```rust
connect!(
    ant.rf_out,
    cable.p1,
    &mut ctx
);
```

表示：

```
Antenna RF output
        |
        |
      Cable
```

---

# 强类型接口检查

每一个硬件接口都有自己的 Rust 类型。

例如：

```rust
Port<SmaMale>

Port<SmaFemale>

Port<NMale>

Port<NFemale>

Port<QsfpPlug>

Port<QsfpJack>
```

接口兼容关系通过 Trait 定义：

```rust
impl Compatible<SmaMale, SmaFemale> for () {}
```

因此：

合法连接：

```rust
connect!(sma_male, sma_female);
```

非法连接：

```rust
connect!(sma_male, qsfp_jack);
```

会在编译阶段失败。

不需要运行时检查。

---

# 自动生成系统框图

Wiring Checker 会生成 D2 文件：

```d2
Antenna: {
    RF: {
        label: "RF output"
    }
}

LNA: {
    RF_IN: {
        label: "SMA"
    }
}

Antenna.RF -> LNA.RF_IN
```

然后可以进一步生成：

- SVG
- PNG
- PDF

用于实验设计文档。

---

# BOM 自动统计

组件声明时可以指定数量：

```rust
new_component!(
    lna,
    Lna,
    32,
    "LNA",
    &mut ctx
);
```

程序自动统计：

```
LNA, 32
N-N同轴线, 32
SMA-SMA线, 64
Bias-T, 32
```

方便实验准备和采购。

---

# 当前支持组件

目前主要面向射频和射电天文前端系统：

## RF

- 天线
- LNA
- 滤波器
- Bias-T
- 同轴电缆
- 转接头

## Power

- AC/DC 电源
- 双绞线
- 电源分配器

## Clock

- GPSDO
- 10 MHz / PPS 分配器
- 时钟电缆

## Optical

- SFP 光模块
- QSFP 光模块
- LC 光纤

## Digital

- FPGA 采集设备接口

---

# 设计原则

## Rust 是系统模型

不是：

```
D2 文件
   |
   v
硬件系统
```

而是：

```
Rust Hardware Model
        |
        v
D2 Visualization
```

---

## 简单优先

Wiring Checker 不试图成为完整 CAD 软件。

它关注的是：

- 接口是否合法
- 连接是否正确
- 物料是否完整
- 系统结构是否清晰

---

# 未来计划

未来可能加入：

## 物理属性检查

例如：

- 工作频率
- 阻抗
- 电压范围
- 数据速率
- 信号方向

例如：

```
50Ω RF output
        |
        |
75Ω input

Error:
impedance mismatch
```

---

## 自动生成文档

包括：

- 接线表
- BOM
- 实验准备清单
- 系统接口说明

---

# 项目状态

Wiring Checker 当前处于早期开发阶段。

欢迎贡献新的：

- Component
- Connector
- Compatibility rule
- 系统模型
