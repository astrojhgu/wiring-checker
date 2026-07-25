use std::io::Write;
use wiring::{
    Connector, component,
    components::{
        AcCable, Ant, BiasT, ClkDistributor, EightWaySplitter, FourWaySplitter, GpsDO, LcFiber,
        Lna, NMale2NMaleCable, NSmaAdapter, QsfpMod, RFFilter, RawTwistedPairCable, SfpMod,
        SmaM2SmaMCable, SmaM2SsmcMCable, T510,
    },
    connect,
    d2::{D2Context, NodeGuard},
    new_component,
    ports::ScrewTerminal,
};
component!(Pwr5V {
    ac_in: ScrewTerminal,
    dc_out: ScrewTerminal
});
component!(Pwr12V {
    ac_in: ScrewTerminal,
    dc_out: ScrewTerminal
});

fn main() {
    // let sma_m = Port::<SmaMale>::new("LNA.OUT".into());

    // let sma_f = Port::<SmaFemale>::new("CABLE.IN".into());

    // sma_m.connect(&sma_f);
    let mut ctx = &mut D2Context::new(std::fs::File::create("a.d2").unwrap());
    new_component!(ant, Ant, &mut ctx, "天线x16");
    new_component!(lna1, Lna, &mut ctx, "LNA1x32");
    new_component!(cable1, NMale2NMaleCable, &mut ctx, "N-N同轴x32");
    new_component!(pwr_5v, Pwr5V, &mut ctx, "5V ACDCx1");
    new_component!(n_sma_adapter, NSmaAdapter, &mut ctx, "N-Sma转接器x32");
    new_component!(sma2sma_cable1, SmaM2SmaMCable, &mut ctx, "sma-sma线x32");
    new_component!(sma2sma_cable2, SmaM2SmaMCable, &mut ctx, "sma-sma线x32");
    new_component!(filter, RFFilter, &mut ctx, "滤波器x32");
    new_component!(pwr_12v, Pwr12V, &mut ctx, "12V ACDC");
    new_component!(ac_cable_for_12v, AcCable, &mut ctx, "交流供电线");
    new_component!(ac_cable_for_5v, AcCable, &mut ctx, "交流供电线");

    new_component!(
        eight_way_splitter_a,
        EightWaySplitter,
        &mut ctx,
        "1进8出分线器x1"
    );
    new_component!(
        four_way_splitter_a,
        FourWaySplitter,
        &mut ctx,
        "1进4出分线器x8"
    );
    new_component!(dc_cable_a1, RawTwistedPairCable, &mut ctx, "双绞线x1");
    new_component!(dc_cable_a2, RawTwistedPairCable, &mut ctx, "双绞线x8");
    new_component!(dc_cable_a3, RawTwistedPairCable, &mut ctx, "双绞线x32");

    new_component!(
        eight_way_splitter_b,
        EightWaySplitter,
        &mut ctx,
        "1进8出分线器x1"
    );
    new_component!(
        four_way_splitter_b,
        FourWaySplitter,
        &mut ctx,
        "1进4出分线器x8"
    );
    new_component!(dc_cable_b1, RawTwistedPairCable, &mut ctx, "双绞线x1");
    new_component!(dc_cable_b2, RawTwistedPairCable, &mut ctx, "双绞线x8");
    new_component!(dc_cable_b3, RawTwistedPairCable, &mut ctx, "双绞线x32");
    new_component!(sma_ssmc, SmaM2SsmcMCable, &mut ctx, "sma-ssmc线x32");
    new_component!(clk_distributor, ClkDistributor, &mut ctx, "10M/pps分配器");
    new_component!(sma_sma_cable_clk_1, SmaM2SmaMCable, &mut ctx, "sma-sma线x1");
    new_component!(sma_sma_cable_pps_1, SmaM2SmaMCable, &mut ctx, "sma-sma线x1");
    new_component!(
        sma_ssmc_cable_clk_2,
        SmaM2SsmcMCable,
        &mut ctx,
        "sma-ssmc线x4"
    );
    new_component!(
        sma_ssmc_cable_pps_2,
        SmaM2SsmcMCable,
        &mut ctx,
        "sma-ssmc线x4"
    );

    new_component!(bias_t, BiasT, &mut ctx, "Bias-Tx32");

    new_component!(t510, T510, &mut ctx, "T510x4");
    new_component!(gpsdo, GpsDO, &mut ctx, "时钟源x1");
    new_component!(sfp_mod_a, SfpMod, &mut ctx, "千兆sfp光模块x4");
    new_component!(qsfp_mod_a, QsfpMod, &mut ctx, "100G Qsfp光模块x4");
    new_component!(gbe_fiber, LcFiber, &mut ctx, "千兆单模光纤x4");
    new_component!(cgbe_fiber, LcFiber, &mut ctx, "100G单模光纤x4");

    new_component!(sfp_mod_b, SfpMod, &mut ctx, "千兆sfp光模块x4");
    new_component!(qsfp_mod_b, QsfpMod, &mut ctx, "100G Qsfp光模块x4");

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
}
