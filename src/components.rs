use crate::{component, ports::*};

component! {
    Lna{
        rf_in: (SmaFemale,1),
        rf_out: (SmaFemale,1),
        pwr: (ScrewTerminal, 1),
    }
}

component! {
    Ant{
        rf_out: (NFemale,1)
    }
}

component! {
    BiPolAnt{
        rf_out: (NFemale,2)
    }
}

component! {
    MwaAnt{
        rf_out: (SmaFemale,2)
    }
}

component! {
    MwaAntSinglePol{
        rf_out: (SmaFemale, 1)
    }
}

component! {
    NMale2NMaleCable{
        p1: (NMale,1),
        p2: (NMale,1),
    }
}

component! {
    SmaM2SmaMCable{
        p1: (SmaMale,1),
        p2: (SmaMale,1),
    }
}

component! {
    SmaM2SsmcMCable{
        p1: (SmaMale, 1),
        p2: (SsmcMale,1),
    }
}

component! {
    BiasT{
        dc: (ScrewTerminal,1),
        rf: (NFemale,1),
        rf_dc: (NFemale,1),
    }
}

component! {
    DCCable{
        input: (DirectContact,1),
        output: (DCPlug,1),
    }
}

component! {
    NSmaAdapter{
        n_side: (NMale,1),
        sma_side: (SmaFemale,1),
    }
}

component! {
    RFFilter{
        input: (SmaFemale,1),
        output: (SmaFemale,1),
    }
}
component! {
    RawTwistedPairCable{
        p1: (DirectContact,1),
        p2: (DirectContact,1),
    }
}

component! {
    FourWaySplitter{
        input: (DirectContact,1),
        output: (DirectContact,4),
    }
}

component! {
    EightWaySplitter{
        input: (DirectContact,1),
        output: (DirectContact,8),
    }
}

component! {
    T510{
        rf_in:(SsmcFemale,8),
        pps_in:(SsmcFemale,1),
        ref_in: (SsmcFemale,1),
        sfp: (SFPCage,1),
        qsfp:(QSFPCage,1),
    }
}

component! {
    GpsDO{
        pps_output: (SmaFemale,1),
        clk_output: (SmaFemale,1),
    }
}

component! {
    ClkDistributor{
        pps_input: (SmaFemale,1),
        clk_input: (SmaFemale,1),
        pps_output: (SmaFemale,4),
        clk_output: (SmaFemale,4),
    }
}

component! {
    SfpMod{
        sfp_plug: (SfpPlug,1),
        lc_jack: (LcJack,1),
    }
}
component! {
    QsfpMod{
        qsfp_plug: (QsfpPlug,1),
        lc_jack: (LcJack,1),
    }
}

component! {
    LcFiber{
        p1: (LcPlug,1),
        p2: (LcPlug,1),
    }
}
component! {
    AcCable{
        p1: (ACPlug,1),
        p2: (DirectContact,1),
    }
}
