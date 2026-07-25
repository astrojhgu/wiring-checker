use crate::{
    Connector, component,
    d2::{D2Context, NodeGuard},
    ports::*,
};
use std::io::Write;

component! {
    Lna{
        rf_in: SmaFemale,
        rf_out: SmaFemale,
        pwr: ScrewTerminal,
    }
}

component! {
    Ant{
        rf_out: NFemale
    }
}

component! {
    BiPolAnt{
        rf_x_out: NFemale,
        rf_y_out: NFemale
    }
}

component! {
    NMale2NMaleCable{
        p1: NMale,
        p2: NMale,
    }
}

component! {
    SmaM2SmaMCable{
        p1: SmaMale,
        p2: SmaMale,
    }
}

component! {
    SmaM2SsmcMCable{
        p1: SmaMale,
        p2: SsmcMale,
    }
}

component! {
    BiasT{
        dc: ScrewTerminal,
        rf: NFemale,
        rf_dc: NFemale,
    }
}

component! {
    DCCable{
        input: DirectContact,
        output: DCPlug,
    }
}

component! {
    NSmaAdapter{
        n_side: NMale,
        sma_side: SmaFemale,
    }
}

component! {
    RFFilter{
        input: SmaFemale,
        output: SmaFemale,
    }
}
component! {
    RawTwistedPairCable{
        p1: DirectContact,
        p2: DirectContact,
    }
}

component! {
    FourWaySplitter{
        input: DirectContact,
        output: DirectContact,
    }
}

component! {
    EightWaySplitter{
        input: DirectContact,
        output: DirectContact,
    }
}

component! {
    T510{
        rf_in:SsmcFemale,
        pps_in:SsmcFemale,
        ref_in: SsmcFemale,
        sfp: SFPCage,
        qsfp:QSFPCage,
    }
}

component! {
    GpsDO{
        pps_output: SmaFemale,
        clk_output: SmaFemale,
    }
}

component! {
    ClkDistributor{
        pps_input: SmaFemale,
        clk_input: SmaFemale,
        pps_output: SmaFemale,
        clk_output: SmaFemale,
    }
}

component! {
    SfpMod{
        sfp_plug: SfpPlug,
        lc_jack: LcJack,
    }
}
component! {
    QsfpMod{
        qsfp_plug: QsfpPlug,
        lc_jack: LcJack,
    }
}

component! {
    LcFiber{
        p1: LcPlug,
        p2: LcPlug,
    }
}
component! {
    AcCable{
        p1: ACPlug,
        p2: DirectContact,
    }
}
