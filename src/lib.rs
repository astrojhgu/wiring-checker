use std::{io::Write, marker::PhantomData};

use crate::d2::D2Context;

pub mod components;
pub mod d2;
pub mod ports;

#[macro_export]
macro_rules! component {
    (
        $name:ident {
            $(
                $port:ident : $ty:ident
            ),* $(,)?
        }
    ) => {
        pub struct $name {
            pub name: String,
            pub description: String,
            $(
                pub $port: Connector<$ty>,
            )*
        }

        impl $name {
            pub fn new<W: Write>(
                ctx: &mut D2Context<W>,
                name: &str,
                description: &str,
            ) -> Self {
                let node = NodeGuard::new(
                    ctx,
                    stringify!($name),
                    name,
                    description
                );

                $(
                    node.ctx
                        .writeln(
                            &format!(
                                r#"
                                {} {{
                                    label: "{}"
                                }}
                                "#,
                                stringify!($port),
                                stringify!($port),
                            )
                        )
                        .unwrap();
                )*

                Self {
                    name: name.into(),
                    description:description.into(),
                    $(
                        $port: Connector::<$ty>::new(),
                    )*
                }
            }
        }
    };
}
#[macro_export]
macro_rules! new_component {
    ($var:ident, $ty:ident, $ctx:expr, $desc:expr) => {
        let $var = $ty::new($ctx, stringify!($var), $desc);
    };
}

pub trait Connectable {}
pub trait Compatible<A: Connectable, B: Connectable> {}

#[derive(Clone, Copy)]
pub struct Connector<C: Copy + Clone + Connectable> {
    _marker: std::marker::PhantomData<C>,
}
impl<C: Copy + Clone + Connectable> Connector<C> {
    pub fn connect_to<B, W>(
        &self,
        _other: Connector<B>,
        self_name: &str,
        other_name: &str,
        ctx: &mut D2Context<W>,
    ) where
        B: Clone + Copy + Connectable,
        (): Compatible<C, B>,
        W: Write,
    {
        ctx.add_edge(self_name, other_name);
    }

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
