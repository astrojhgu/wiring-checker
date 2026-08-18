#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::{io::Write, marker::PhantomData};

use crate::d2::D2Context;

pub mod components;
pub mod d2;
pub mod ports;

/// Compile-time port-count equality check.
/// `Eq<A, B>: EqCheck` is satisfied only when `A == B`.
pub struct Eq<const A: usize, const B: usize>;
pub trait EqCheck {}
impl<const N: usize> EqCheck for Eq<N, N> {}

/// A typed group of ports.
/// - `M`: number of devices
/// - `N`: ports per device
/// - `TOTAL`: total port count = `M * N` (computed at the call site, avoids const-expr in where bounds)
#[derive(Clone, Copy)]
pub struct Group<T: Copy + Clone, const M: usize, const N: usize, const TOTAL: usize> {
    _obj: T,
    _marker: PhantomData<T>,
}

impl<T: Copy + Clone, const M: usize, const N: usize, const TOTAL: usize> Group<T, M, N, TOTAL> {
    pub fn new(x: T) -> Self {
        Self {
            _obj: x,
            _marker: PhantomData::default(),
        }
    }
}

impl<C: Copy + Clone + Connectable, const M: usize, const N: usize, const TOTAL: usize>
    Group<Connector<C>, M, N, TOTAL>
{
    pub fn connect_to<B, W, const P: usize, const Q: usize, const TOTAL_R: usize>(
        &self,
        _other: Group<Connector<B>, P, Q, TOTAL_R>,
        self_name: &str,
        other_name: &str,
        ctx: &mut D2Context<W>,
    ) where
        B: Clone + Copy + Connectable,
        (): Compatible<C, B>,
        W: Write,
        Eq<TOTAL, TOTAL_R>: EqCheck,
    {
        ctx.add_edge(self_name, other_name);
    }
}

#[macro_export]
macro_rules! component {
    (
        $name:ident {
            $(
                $port:ident : ($ty:ident, $n: literal)
            ),* $(,)?
        }
    ) => {
        pub struct $name <const M: usize>
        where
            $(
                [(); { M * $n }]: Sized,
            )*
        {
            pub name: String,
            pub description: String,
            $(
                pub $port: Group<Connector<$ty>, M, $n, { M * $n }>,
            )*
        }

        impl<const M:usize> $name<M>
        where
            $(
                [(); { M * $n }]: Sized,
            )*
        {
            pub fn new<W: Write>(
                ctx: &mut D2Context<W>,
                name: &str,
                description: &str,
            ) -> Self {
                (*ctx.bom.entry(description.to_string()).or_insert(0))+=M;
                let node = NodeGuard::new(
                    ctx,
                    stringify!($name),
                    name,
                    format!("{} x {}",description, M).as_str()
                );


                $(
                    node.ctx
                        .writeln(
                            &format!(
                                r#"
                                {} {{
                                    label: "{} x {}"
                                }}
                                "#,
                                stringify!($port),
                                stringify!($port),
                                $n
                            )
                        )
                        .unwrap();
                )*

                Self {
                    name: name.into(),
                    description:description.into(),
                    $(
                        $port: Group::new(Connector::<$ty>::new()),
                    )*
                }
            }
        }
    };
}
#[macro_export]
macro_rules! new_component {
    ($var:ident, $ty:ident, $qty:expr, $desc:expr, $ctx:expr) => {
        let $var = $ty::<{ $qty }>::new($ctx, stringify!($var), $desc);
    };
}

pub trait Connectable {}
pub trait Compatible<A: Connectable, B: Connectable> {}

#[derive(Clone, Copy)]
pub struct Connector<C: Copy + Clone + Connectable> {
    _marker: std::marker::PhantomData<C>,
}
impl<C: Copy + Clone + Connectable> Connector<C> {
    // pub fn connect_to<B, W>(
    //     &self,
    //     _other: Connector<B>,
    //     self_name: &str,
    //     other_name: &str,
    //     ctx: &mut D2Context<W>,
    // ) where
    //     B: Clone + Copy + Connectable,
    //     (): Compatible<C, B>,
    //     W: Write,
    // {
    //     ctx.add_edge(self_name, other_name);
    // }

    pub fn new() -> Connector<C> {
        Connector {
            _marker: PhantomData::default(),
        }
    }
}

#[macro_export]
macro_rules! connect {
    ($a:expr, $b:expr, $ctx:expr) => {
        $a.connect_to($b, stringify!($a), stringify!($b), $ctx);
    };
}
