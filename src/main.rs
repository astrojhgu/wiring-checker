use std::io::Write;
use wiring_checker::{
    Connector, Group, component,
    components::{
        AcCable, BiPolAnt, BiasT, ClkDistributor, EightWaySplitter, FourWaySplitter, GpsDO,
        LcFiber, Lna, NMale2NMaleCable, NSmaAdapter, QsfpMod, RFFilter, RawTwistedPairCable,
        SfpMod, SmaM2SmaMCable, SmaM2SsmcMCable, T510,
    },
    connect,
    d2::{D2Context, NodeGuard},
    new_component,
    ports::ScrewTerminal,
};
component!(Pwr5V {
    ac_in: (ScrewTerminal, 1),
    dc_out: (ScrewTerminal, 1)
});
component!(Pwr12V {
    ac_in: (ScrewTerminal, 1),
    dc_out: (ScrewTerminal, 1)
});

fn main() {
    // let sma_m = Port::<SmaMale>::new("LNA.OUT".into());

    // let sma_f = Port::<SmaFemale>::new("CABLE.IN".into());

    // sma_m.connect(&sma_f);
    let mut ctx = &mut D2Context::new(std::fs::File::create("a.d2").unwrap());
    new_component!(ant, BiPolAnt, 16, "天线", &mut ctx);
    new_component!(lna1, Lna, 32, "LNA1", &mut ctx);
    new_component!(cable1, NMale2NMaleCable, 32, "N-N同轴线", &mut ctx);
    new_component!(pwr_5v, Pwr5V, 1, "5V ACDC", &mut ctx);
    new_component!(n_sma_adapter, NSmaAdapter, 32, "N-Sma转接器", &mut ctx);
    new_component!(sma2sma_cable1, SmaM2SmaMCable, 32, "sma-sma线", &mut ctx);
    new_component!(sma2sma_cable2, SmaM2SmaMCable, 32, "sma-sma线", &mut ctx);
    new_component!(filter, RFFilter, 32, "滤波器", &mut ctx);
    new_component!(pwr_12v, Pwr12V, 1, "12V ACDC", &mut ctx);
    new_component!(ac_cable_for_12v, AcCable, 1, "交流供电线", &mut ctx);
    new_component!(ac_cable_for_5v, AcCable, 1, "交流供电线", &mut ctx);

    new_component!(
        eight_way_splitter_a,
        EightWaySplitter,
        1,
        "1进8出分线器",
        &mut ctx
    );
    new_component!(
        four_way_splitter_a,
        FourWaySplitter,
        8,
        "1进4出分线器",
        &mut ctx
    );
    new_component!(dc_cable_a1, RawTwistedPairCable, 1, "双绞线", &mut ctx);
    new_component!(dc_cable_a2, RawTwistedPairCable, 8, "双绞线", &mut ctx);
    new_component!(dc_cable_a3, RawTwistedPairCable, 32, "双绞线", &mut ctx);

    new_component!(
        eight_way_splitter_b,
        EightWaySplitter,
        1,
        "1进8出分线器",
        &mut ctx
    );
    new_component!(
        four_way_splitter_b,
        FourWaySplitter,
        8,
        "1进4出分线器",
        &mut ctx
    );
    new_component!(dc_cable_b1, RawTwistedPairCable, 1, "双绞线", &mut ctx);
    new_component!(dc_cable_b2, RawTwistedPairCable, 8, "双绞线", &mut ctx);
    new_component!(dc_cable_b3, RawTwistedPairCable, 32, "双绞线", &mut ctx);
    new_component!(sma_ssmc, SmaM2SsmcMCable, 32, "sma-ssmc线", &mut ctx);
    new_component!(
        clk_distributor,
        ClkDistributor,
        1,
        "10M/pps分配器",
        &mut ctx
    );
    new_component!(
        sma_sma_cable_clk_1,
        SmaM2SmaMCable,
        1,
        "sma-sma线",
        &mut ctx
    );
    new_component!(
        sma_sma_cable_pps_1,
        SmaM2SmaMCable,
        1,
        "sma-sma线",
        &mut ctx
    );
    new_component!(
        sma_ssmc_cable_clk_2,
        SmaM2SsmcMCable,
        4,
        "sma-ssmc线",
        &mut ctx
    );
    new_component!(
        sma_ssmc_cable_pps_2,
        SmaM2SsmcMCable,
        4,
        "sma-ssmc线",
        &mut ctx
    );

    new_component!(bias_t, BiasT, 32, "Bias-T", &mut ctx);

    new_component!(t510, T510, 4, "T510", &mut ctx);
    new_component!(gpsdo, GpsDO, 1, "时钟源", &mut ctx);
    new_component!(sfp_mod_a, SfpMod, 4, "sfp光模块", &mut ctx);
    new_component!(qsfp_mod_a, QsfpMod, 4, "100G Qsfp光模块", &mut ctx);
    new_component!(gbe_fiber, LcFiber, 4, "单模光纤", &mut ctx);
    new_component!(cgbe_fiber, LcFiber, 4, "单模光纤", &mut ctx);

    new_component!(sfp_mod_b, SfpMod, 4, "sfp光模块", &mut ctx);
    new_component!(qsfp_mod_b, QsfpMod, 4, "100G Qsfp光模块", &mut ctx);

    connect!(ant.rf_out, cable1.p1, &mut ctx);

    connect!(cable1.p2, bias_t.rf_dc, &mut ctx);
    //connect!(pwr_5v.dc_out, bias_t.dc, &mut ctx);
    connect!(pwr_5v.dc_out, dc_cable_a1.p1, &mut ctx);
    connect!(dc_cable_a1.p2, eight_way_splitter_a.input, &mut ctx);
    connect!(eight_way_splitter_a.output, dc_cable_a2.p1, &mut ctx);
    connect!(dc_cable_a2.p2, four_way_splitter_a.input, &mut ctx);
    connect!(four_way_splitter_a.output, dc_cable_a3.p1, &mut ctx);
    connect!(dc_cable_a3.p2, bias_t.dc, &mut ctx);
    connect!(bias_t.rf, n_sma_adapter.n_side, &mut ctx);
    connect!(n_sma_adapter.sma_side, sma2sma_cable1.p1, &mut ctx);
    connect!(sma2sma_cable1.p2, filter.input, &mut ctx);
    connect!(filter.output, sma2sma_cable2.p1, &mut ctx);
    connect!(sma2sma_cable2.p2, lna1.rf_in, &mut ctx);

    connect!(pwr_12v.dc_out, dc_cable_b1.p1, &mut ctx);
    connect!(dc_cable_b1.p2, eight_way_splitter_b.input, &mut ctx);
    connect!(eight_way_splitter_b.output, dc_cable_b2.p1, &mut ctx);
    connect!(dc_cable_b2.p2, four_way_splitter_b.input, &mut ctx);
    connect!(four_way_splitter_b.output, dc_cable_b3.p1, &mut ctx);
    connect!(dc_cable_b3.p2, lna1.pwr, &mut ctx);
    connect!(lna1.rf_out, sma_ssmc.p1, &mut ctx);
    connect!(sma_ssmc.p2, t510.rf_in, &mut ctx);
    connect!(gpsdo.clk_output, sma_sma_cable_clk_1.p1, &mut ctx);
    connect!(sma_sma_cable_clk_1.p2, clk_distributor.clk_input, &mut ctx);

    connect!(gpsdo.pps_output, sma_sma_cable_pps_1.p1, &mut ctx);
    connect!(sma_sma_cable_pps_1.p2, clk_distributor.pps_input, &mut ctx);

    connect!(
        clk_distributor.clk_output,
        sma_ssmc_cable_clk_2.p1,
        &mut ctx
    );
    connect!(
        clk_distributor.pps_output,
        sma_ssmc_cable_pps_2.p1,
        &mut ctx
    );

    connect!(sma_ssmc_cable_clk_2.p2, t510.ref_in, &mut ctx);
    connect!(sma_ssmc_cable_pps_2.p2, t510.pps_in, &mut ctx);
    connect!(t510.sfp, sfp_mod_a.sfp_plug, &mut ctx);
    connect!(t510.qsfp, qsfp_mod_a.qsfp_plug, &mut ctx);
    connect!(sfp_mod_a.lc_jack, gbe_fiber.p1, &mut ctx);
    connect!(qsfp_mod_a.lc_jack, cgbe_fiber.p1, &mut ctx);

    connect!(gbe_fiber.p2, sfp_mod_b.lc_jack, &mut ctx);
    connect!(cgbe_fiber.p2, qsfp_mod_b.lc_jack, &mut ctx);

    connect!(ac_cable_for_12v.p2, pwr_12v.ac_in, &mut ctx);
    connect!(ac_cable_for_5v.p2, pwr_5v.ac_in, &mut ctx);

    for (k, v) in ctx.bom.iter() {
        println!("{k}: {v}");
    }
}
