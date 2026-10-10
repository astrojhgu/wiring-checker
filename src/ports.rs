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
define_connector_pair!(ACPlug, ACJack);
// 国标三眼插头。它插的墙上插座不在系统模型里，所以没有对端类型。
define_connector!(MainsPlug);
define_connector_pair!(Lp20Plug, Lp20Jack);
define_connector_pair!(Rj45Plug, Rj45Jack);
define_connector_pair!(HdmiPlug, HdmiJack);
define_connector_pair!(UsbAPlug, UsbAJack);
define_connector_pair!(UsbCPlug, UsbCJack);
