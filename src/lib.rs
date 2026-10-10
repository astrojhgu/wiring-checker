#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::marker::PhantomData;

pub mod components;
pub mod ctx;
pub mod d2;
pub mod output;
pub mod ports;

pub use ctx::Ctx;

/// 编译期端口总数相等检查：`Eq<A, B>: EqCheck` 仅在 `A == B` 时成立。
pub struct Eq<const A: usize, const B: usize>;
pub trait EqCheck {}
impl<const N: usize> EqCheck for Eq<N, N> {}

/// 一组同型号端口。
/// - `M`：器件数
/// - `N`：每器件端口数
/// - `TOTAL`：端口总数 `M * N`，在调用点算好，绕开 where 子句里的 const 表达式
///
/// 故意不实现 `Copy`/`Clone`：一个端口组只能被连接一次。重复接线是编译错误，
/// 不是运行时断言；漏接线由 `Ctx::verify` 兜底。
pub struct Group<T: Copy + Clone, const M: usize, const N: usize, const TOTAL: usize> {
    _marker: PhantomData<T>,
}

impl<T: Copy + Clone, const M: usize, const N: usize, const TOTAL: usize> Group<T, M, N, TOTAL> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<C: Copy + Clone + Connectable, const M: usize, const N: usize, const TOTAL: usize>
    Group<Connector<C>, M, N, TOTAL>
{
    /// 把两组端口整体对接。两组端口总数必须相等（编译期检查），
    /// 连接后两个端口组即被消耗，不能再接第二次。
    pub fn connect_to<B, W, const P: usize, const Q: usize, const TOTAL_R: usize>(
        self,
        _other: Group<Connector<B>, P, Q, TOTAL_R>,
        self_name: &str,
        other_name: &str,
        ctx: &mut Ctx<W>,
    ) where
        B: Copy + Clone + Connectable,
        (): Compatible<C, B>,
        W: std::io::Write,
        Eq<TOTAL, TOTAL_R>: EqCheck,
    {
        ctx.consume(self_name, TOTAL);
        ctx.consume(other_name, TOTAL_R);
        ctx.add_edge(self_name, other_name, TOTAL);
    }
}

/// 定义一个组件类型。每个端口写成 `端口名: (接头类型, 每器件端口数)`。
#[macro_export]
macro_rules! component {
    (
        $name:ident {
            $(
                $port:ident : ($ty:ident, $n: literal)
            ),* $(,)?
        }
    ) => {
        pub struct $name<const M: usize>
        where
            $(
                [(); { M * $n }]: Sized,
            )*
        {
            pub name: String,
            pub description: String,
            $(
                pub $port: $crate::Group<$crate::Connector<$ty>, M, $n, { M * $n }>,
            )*
        }

        impl<const M: usize> $name<M>
        where
            $(
                [(); { M * $n }]: Sized,
            )*
        {
            pub fn new<W: ::std::io::Write>(
                ctx: &mut $crate::Ctx<W>,
                name: &str,
                description: &str,
            ) -> Self {
                ctx.add_item(name, description, M);
                let node = $crate::d2::NodeGuard::new(
                    ctx,
                    name,
                    &format!("{} x {}", description, M),
                );
                $(
                    node.ctx
                        .declare(&format!("{}.{}", name, stringify!($port)), { M * $n });
                    node.ctx
                        .writeln(&format!(
                            r#"
                                {} {{
                                    label: "{} x {}"
                                }}
                                "#,
                            stringify!($port),
                            stringify!($port),
                            $n
                        ))
                        .unwrap();
                )*
                drop(node);

                Self {
                    name: name.into(),
                    description: description.into(),
                    $(
                        $port: $crate::Group::new(),
                    )*
                }
            }
        }
    };
}

/// 声明一批器件。`数量` 是器件台数，端口总数由组件定义乘以台数得到。
#[macro_export]
macro_rules! new_component {
    ($var:ident, $ty:ident, $qty:expr, $desc:expr, $ctx:expr) => {
        let $var = $ty::<{ $qty }>::new($ctx, stringify!($var), $desc);
    };
}

pub trait Connectable {}
pub trait Compatible<A: Connectable, B: Connectable> {}

/// 接头类型的类型标记，只在类型层面存在。
#[derive(Clone, Copy)]
pub struct Connector<C: Copy + Clone + Connectable> {
    _marker: PhantomData<C>,
}

/// 把两个端口组整体对接。端口总数不等时编译失败。
#[macro_export]
macro_rules! connect {
    ($a:expr, $b:expr, $ctx:expr) => {
        $a.connect_to($b, stringify!($a), stringify!($b), $ctx);
    };
}
