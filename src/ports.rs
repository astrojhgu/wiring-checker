use crate::{Compatible, Connectable};

macro_rules! define_connector {
    ($name:ident) => {
        #[derive(Copy, Clone)]
        pub struct $name {}

        impl $name {
            pub fn new() -> $name {
                $name {}
            }
        }

        impl Connectable for $name {}
    };
}

macro_rules! define_connector_pair {
    ($a:ident, $b:ident) => {
        define_connector!($a);
        define_connector!($b);

        impl Compatible<$a, $b> for () {}
        impl Compatible<$b, $a> for () {}
    };
}

define_connector_pair!(SmaMale, SmaFemale);
define_connector_pair!(NMale, NFemale);
define_connector_pair!(DCJack, DCPlug);
define_connector!(DirectContact);
impl Compatible<DirectContact, DirectContact> for () {}
define_connector!(ScrewTerminal);
impl Compatible<ScrewTerminal, DirectContact> for () {}
impl Compatible<DirectContact, ScrewTerminal> for () {}
define_connector_pair!(SsmcMale, SsmcFemale);
define_connector_pair!(SfpPlug, SFPCage);
define_connector_pair!(QsfpPlug, QSFPCage);
define_connector_pair!(LcPlug, LcJack);
define_connector!(ACPlug);
