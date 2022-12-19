use std::collections::HashMap;

// cf. https://www.promotic.eu/en/pmdoc/Subsystems/Comm/PmDrivers/IEC62056_OBIS.htm
use enum_iterator::all;
use lazy_static::lazy_static;

// HashMap<[u8;6], Obis>
lazy_static! {
    static ref SOURCE: HashMap<&'static [u8], Obis> = {
        let mut a = HashMap::new();
        for obis in all::<Obis>() {
            a.insert(obis.clone().obis_number(), obis);
        }
        a
    };
}
macro_rules! generate_obis {

     ($( ($x:ident, $y:expr, $l:literal) ),*) => {
        #[derive(serde::Serialize, serde::Deserialize, enum_iterator::Sequence, Eq, PartialEq, Hash, Clone)]
        #[non_exhaustive]
        pub enum Obis {
             $(
                #[doc = $l]
                 $x,
             )*
         }

        impl Obis {
             pub fn obis_number(&self) -> &'static [u8] {
                 match self {
                    $(
                        Self:: $x => $y,
                    )*
                 }
             }

             /// Find matching Obis number from six-digit number
             pub fn from_number(number: &[u8]) -> Option<Self> {
                SOURCE.get(number).map(|x| x.clone())
             }
         }
    };
}

generate_obis! {
    (PositiveActiveEnergyTotal, &[1, 0, 1, 8, 0,255],"Positive active energy (A+) total [kWh]"),
    (PositiveActiveEnergyTarif1, &[1, 0, 1, 8, 1,255],"Positive active energy (A+) in tariff T1 [kWh]"),
    (PositiveActiveEnergyTarif2, &[1, 0, 1, 8, 2,255],"Positive active energy (A+) in tariff T2 [kWh]"),
    (PositiveActiveEnergyTarif3, &[1, 0, 1, 8, 3,255],"Positive active energy (A+) in tariff T3 [kWh]"),
    (PositiveActiveEnergyTarif4, &[1, 0, 1, 8, 4,255],"Positive active energy (A+) in tariff T4 [kWh]"),
    (NegativeActiveEnergyTotal, &[1, 0, 2, 8, 0,255],"Negative active energy (A+) total [kWh]"),
    (NegativeActiveEnergyTarif1, &[1, 0, 2, 8, 1,255],"Negative active energy (A+) in tariff T1 [kWh]"),
    (NegativeActiveEnergyTarif2, &[1, 0, 2, 8, 2,255],"Negative active energy (A+) in tariff T2 [kWh]"),
    (NegativeActiveEnergyTarif3, &[1, 0, 2, 8, 3,255],"Negative active energy (A+) in tariff T3 [kWh]"),
    (NegativeActiveEnergyTarif4, &[1, 0, 2, 8, 4,255],"Negative active energy (A+) in tariff T4 [kWh]"),
    (AbsoluteActiveEnergyTotal, &[1, 0, 15, 8, 0,255],"Absolute active energy (A+) total [kWh]"),
    (AbsoluteActiveEnergyTarif1, &[1, 0, 15, 8, 1,255],"Absolute active energy (A+) in tariff T1 [kWh]"),
    (AbsoluteActiveEnergyTarif2, &[1, 0, 15, 8, 2,255],"Absolute active energy (A+) in tariff T2 [kWh]"),
    (AbsoluteActiveEnergyTarif3, &[1, 0, 15, 8, 3,255],"Absolute active energy (A+) in tariff T3 [kWh]"),
    (AbsoluteActiveEnergyTarif4, &[1, 0, 15, 8, 4,255],"Absolute active energy (A+) in tariff T4 [kWh]"),
    (SumActiveEnergyWithoutReverseBlockadeTotal, &[1, 0, 16, 8, 0,255],"Sum active energy without reverse blockade (A+ - A-) total [kWh]"),
    (SumActiveEnergyWithoutReverseBlockadeTarif1, &[1, 0, 16, 8, 1,255],"Sum active energy without reverse blockade (A+ - A-) in tariff T1 [kWh]"),
    (SumActiveEnergyWithoutReverseBlockadeTarif2, &[1, 0, 16, 8, 2,255],"Sum active energy without reverse blockade (A+ - A-) in tariff T2 [kWh]"),
    (SumActiveEnergyWithoutReverseBlockadeTarif3, &[1, 0, 16, 8, 3,255],"Sum active energy without reverse blockade (A+ - A-) in tariff T3 [kWh]"),
    (SumActiveEnergyWithoutReverseBlockadeTarif4, &[1, 0, 16, 8, 4,255],"Sum active energy without reverse blockade (A+ - A-) in tariff T4 [kWh]"),
    (PositiveReactiveEnergyTotal, &[1, 0, 3, 8, 0,255],"Positive reactive energy (Q+) total [kvarh]"),
    (PositiveReactiveEnergyTarif1, &[1, 0, 3, 8, 1,255],"Positive reactive energy (Q+) in tariff T1 [kvarh]"),
    (PositiveReactiveEnergyTarif2, &[1, 0, 3, 8, 2,255],"Positive reactive energy (Q+) in tariff T2 [kvarh]"),
    (PositiveReactiveEnergyTarif3, &[1, 0, 3, 8, 3,255],"Positive reactive energy (Q+) in tariff T3 [kvarh]"),
    (PositiveReactiveEnergyTarif4, &[1, 0, 3, 8, 4,255],"Positive reactive energy (Q+) in tariff T4 [kvarh]"),
    (NegativeReactiveEnergyTotal, &[1, 0, 4, 8, 0,255],"Negative reactive energy (Q-) total [kvarh]"),
    (NegativeReactiveEnergyTarif1, &[1, 0, 4, 8, 1,255],"Negative reactive energy (Q-) in tariff T1 [kvarh]"),
    (NegativeReactiveEnergyTarif2, &[1, 0, 4, 8, 2,255],"Negative reactive energy (Q-) in tariff T2 [kvarh]"),
    (NegativeReactiveEnergyTarif3, &[1, 0, 4, 8, 3,255],"Negative reactive energy (Q-) in tariff T3 [kvarh]"),
    (NegativeReactiveEnergyTarif4, &[1, 0, 4, 8, 4,255],"Negative reactive energy (Q-) in tariff T4 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ1Total, &[1, 0, 5, 8, 0,255],"Imported inductive reactive energy in 1-st quadrant (Q1) total [kvarh]"),
    (ImportedInductiveReactiveEnergyQ1Tarif1, &[1, 0, 5, 8, 1,255],"Imported inductive reactive energy in 1-st quadrant (Q1) in tariff T1 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ1Tarif2, &[1, 0, 5, 8, 2,255],"Imported inductive reactive energy in 1-st quadrant (Q1) in tariff T2 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ1Tarif3, &[1, 0, 5, 8, 3,255],"Imported inductive reactive energy in 1-st quadrant (Q1) in tariff T3 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ1Tarif4, &[1, 0, 5, 8, 4,255],"Imported inductive reactive energy in 1-st quadrant (Q1) in tariff T4 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ2Total, &[1, 0, 6, 8, 0,255],"Imported capacitive reactive energy in 2-nd quadrant (Q2) total [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ2Tarif1, &[1, 0, 6, 8, 1,255],"Imported capacitive reactive energy in 2-nd quadr. (Q2) in tariff T1 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ2Tarif2, &[1, 0, 6, 8, 2,255],"Imported capacitive reactive energy in 2-nd quadr. (Q2) in tariff T2 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ2Tarif3, &[1, 0, 6, 8, 3,255],"Imported capacitive reactive energy in 2-nd quadr. (Q2) in tariff T3 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ2Tarif4, &[1, 0, 6, 8, 4,255],"Imported capacitive reactive energy in 2-nd quadr. (Q2) in tariff T4 [kvarh]"),
    (ExportedInductiveReactiveEnergyQ3Total, &[1, 0, 7, 8, 0,255],"Exported inductive reactive energy in 3-rd quadrant (Q3) total [kvarh]"),
    (ImportedInductiveReactiveEnergyQ3Tarif1, &[1, 0, 7, 8, 1,255],"Exported inductive reactive energy in 3-rd quadrant (Q3) in tariff T1 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ3Tarif2, &[1, 0, 7, 8, 2,255],"Exported inductive reactive energy in 3-rd quadrant (Q3) in tariff T2 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ3Tarif3, &[1, 0, 7, 8, 3,255],"Exported inductive reactive energy in 3-rd quadrant (Q3) in tariff T3 [kvarh]"),
    (ImportedInductiveReactiveEnergyQ3Tarif4, &[1, 0, 7, 8, 4,255],"Exported inductive reactive energy in 3-rd quadrant (Q3) in tariff T4 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ4Total, &[1, 0, 8, 8, 0,255],"Exported capacitive reactive energy in 4-th quadrant (Q4) total [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ4Tarif1, &[1, 0, 8, 8, 1,255],"Exported capacitive reactive energy in 4-th quadr. (Q4) in tariff T1 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ4Tarif2, &[1, 0, 8, 8, 2,255],"Exported capacitive reactive energy in 4-th quadr. (Q4) in tariff T2 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ4Tarif3, &[1, 0, 8, 8, 3,255],"Exported capacitive reactive energy in 4-th quadr. (Q4) in tariff T3 [kvarh]"),
    (ImportedCapacitiveReactiveEnergyQ4Tarif4, &[1, 0, 8, 8, 4,255],"Exported capacitive reactive energy in 4-th quadr. (Q4) in tariff T4 [kvarh]"),
    (ApparentEnergyTotal, &[1, 0, 9, 8, 0,255],"Apparent energy (S+) total [kVAh]"),
    (ApparentEnergyTarif1, &[1, 0, 9, 8, 1,255],"Apparent energy (S+) in tariff T1 [kVAh]"),
    (ApparentEnergyTarif2, &[1, 0, 9, 8, 2,255],"Apparent energy (S+) in tariff T2 [kVAh]"),
    (ApparentEnergyTarif3, &[1, 0, 9, 8, 3,255],"Apparent energy (S+) in tariff T3 [kVAh]"),
    (ApparentEnergyTarif4, &[1, 0, 9, 8, 4,255],"Apparent energy (S+) in tariff T4 [kVAh]"),
    (PositveActiveEnergyPhaseL1Total, &[1, 0, 21, 8, 0,255],"Positive active energy (A+) in phase L1 total [kWh]"),
    (PositveActiveEnergyPhaseL2Total, &[1, 0, 41, 8, 0,255],"Positive active energy (A+) in phase L2 total [kWh]"),
    (PositveActiveEnergyPhaseL3Total, &[1, 0, 61, 8, 0,255],"Positive active energy (A+) in phase L3 total [kWh]"),
    (NegativeActiveEnergyPhaseL1Total, &[1, 0, 22, 8, 0,255],"Negative active energy (A-) in phase L1 total [kWh]"),
    (NegativeActiveEnergyPhaseL2Total, &[1, 0, 42, 8, 0,255],"Negative active energy (A-) in phase L2 total [kWh]"),
    (NegativeActiveEnergyPhaseL3Total, &[1, 0, 62, 8, 0,255],"Negative active energy (A-) in phase L3 total [kWh]"),
    (AbsoluteActiveEnergyPhaseL1Total, &[1, 0, 35, 8, 0,255],"Absolute active energy (|A|) in phase L1 total [kWh]"),
    (AbsoluteActiveEnergyPhaseL2Total, &[1, 0, 55, 8, 0,255],"Absolute active energy (|A|) in phase L2 total [kWh]"),
    (AbsoluteActiveEnergyPhaseL3Total, &[1, 0, 75, 8, 0,255],"Absolute active energy (|A|) in phase L3 total [kWh]"),
    (PositveActiveMaximumDemandTotal, &[1, 0, 1, 6, 0,255],"Positive active maximum demand (A+) total [kW]"),
    (PositveActiveMaximumDemandTarif1, &[1, 0, 1, 6, 1,255],"Positive active maximum demand (A+) in tariff T1 [kW]"),
    (PositveActiveMaximumDemandTarif2, &[1, 0, 1, 6, 2,255],"Positive active maximum demand (A+) in tariff T2 [kW]"),
    (PositveActiveMaximumDemandTarif3, &[1, 0, 1, 6, 3,255],"Positive active maximum demand (A+) in tariff T3 [kW]"),
    (PositveActiveMaximumDemandTarif4, &[1, 0, 1, 6, 4,255],"Positive active maximum demand (A+) in tariff T4 [kW]"),
    (NegativeActiveMaximumDemandTotal, &[1, 0, 2, 6, 0,255],"Negative active maximum demand (A-) total [kW]"),
    (NegativeActiveMaximumDemandTarif1, &[1, 0, 2, 6, 1,255],"Negative active maximum demand (A-) in tariff T1 [kW]"),
    (NegativeActiveMaximumDemandTarif2, &[1, 0, 2, 6, 2,255],"Negative active maximum demand (A-) in tariff T2 [kW]"),
    (NegativeActiveMaximumDemandTarif3, &[1, 0, 2, 6, 3,255],"Negative active maximum demand (A-) in tariff T3 [kW]"),
    (NegativeActiveMaximumDemandTarif4, &[1, 0, 2, 6, 4,255],"Negative active maximum demand (A-) in tariff T4 [kW]"),
    (AbsoluteActiveMaximumDemandTotal, &[1, 0, 15, 6, 0,255],"Absolute active maximum demand (|A|) total [kW]"),
    (AbsoluteActiveMaximumDemandTarif1, &[1, 0, 15, 6, 1,255],"Absolute active maximum demand (|A|) in tariff T1 [kW]"),
    (AbsoluteActiveMaximumDemandTarif2, &[1, 0, 15, 6, 2,255],"Absolute active maximum demand (|A|) in tariff T2 [kW]"),
    (AbsoluteActiveMaximumDemandTarif3, &[1, 0, 15, 6, 3,255],"Absolute active maximum demand (|A|) in tariff T3 [kW]"),
    (AbsoluteActiveMaximumDemandTarif4, &[1, 0, 15, 6, 4,255],"Absolute active maximum demand (|A|) in tariff T4 [kW]"),
    (PositveReactiveMaximumDemandTotal, &[1, 0, 3, 6, 0,255],"Positive reactive maximum demand (Q+) total [kvar]"),
    (NegativeReactiveMaximumDemandTotal, &[1, 0, 4, 6, 0,255],"Negative reactive maximum demand (Q-) total [kvar]"),
    (ReactiveMaximumDemandQ1Total, &[1, 0, 5, 6, 0,255],"Reactive maximum demand in Q1 (Q1) total [kvar]"),
    (ReactiveMaximumDemandQ2Total, &[1, 0, 6, 6, 0,255],"Reactive maximum demand in Q2 (Q2) total [kvar]"),
    (ReactiveMaximumDemandQ3Total, &[1, 0, 7, 6, 0,255],"Reactive maximum demand in Q3 (Q3) total [kvar]"),
    (ReactiveMaximumDemandQ4Total, &[1, 0, 8, 6, 0,255],"Reactive maximum demand in Q4 (Q4) total [kvar]"),
    (ApparentMaximumDemandTotal, &[1, 0, 9, 6, 0,255],"Apparent maximum demand (S+) total [kVA]"),
    (PositiveActiveCumulativeMaximumDemandTotal, &[1, 0, 1, 2, 0,255],"Positive active cumulative maximum demand (A+) total [kW]"),
    (PositiveActiveCumulativeMaximumDemandTarif1, &[1, 0, 1, 2, 1,255],"Positive active cumulative maximum demand (A+) in tariff T1 [kW]"),
    (PositiveActiveCumulativeMaximumDemandTarif2, &[1, 0, 1, 2, 2,255],"Positive active cumulative maximum demand (A+) in tariff T2 [kW]"),
    (PositiveActiveCumulativeMaximumDemandTarif3, &[1, 0, 1, 2, 3,255],"Positive active cumulative maximum demand (A+) in tariff T3 [kW]"),
    (PositiveActiveCumulativeMaximumDemandTarif4, &[1, 0, 1, 2, 4,255],"Positive active cumulative maximum demand (A+) in tariff T4 [kW]"),
    (NegativeActiveCumulativeMaximumDemandTotal, &[1, 0, 2, 2, 0,255],"Negative active cumulative maximum demand (A-) total [kW]"),
    (NegativeActiveCumulativeMaximumDemandTarif1, &[1, 0, 2, 2, 1,255],"Negative active cumulative maximum demand (A-) in tariff T1 [kW]"),
    (NegativeActiveCumulativeMaximumDemandTarif2, &[1, 0, 2, 2, 2,255],"Negative active cumulative maximum demand (A-) in tariff T2 [kW]"),
    (NegativeActiveCumulativeMaximumDemandTarif3, &[1, 0, 2, 2, 3,255],"Negative active cumulative maximum demand (A-) in tariff T3 [kW]"),
    (NegativeActiveCumulativeMaximumDemandTarif4, &[1, 0, 2, 2, 4,255],"Negative active cumulative maximum demand (A-) in tariff T4 [kW]"),
    (AbsoluteActiveCumulativeMaximumDemandTotal, &[1, 0, 15, 2, 0,255],"Absolute active cumulative maximum demand (|A|) total [kW]"),
    (AbsoluteActiveCumulativeMaximumDemandTarif1, &[1, 0, 15, 2, 1,255],"Absolute active cumulative maximum demand (|A|) in tariff T1 [kW]"),
    (AbsoluteActiveCumulativeMaximumDemandTarif2, &[1, 0, 15, 2, 2,255],"Absolute active cumulative maximum demand (|A|) in tariff T2 [kW]"),
    (AbsoluteActiveCumulativeMaximumDemandTarif3, &[1, 0, 15, 2, 3,255],"Absolute active cumulative maximum demand (|A|) in tariff T3 [kW]"),
    (AbsoluteActiveCumulativeMaximumDemandTarif4, &[1, 0, 15, 2, 4,255],"Absolute active cumulative maximum demand (|A|) in tariff T4 [kW]"),
    (PositiveReactiveCumulativeMaximumDemandTotal, &[1, 0, 3, 2, 0,255],"Positive reactive cumulative maximum demand (Q+) total [kvar]"),
    (NegativeReactiveCumulativeMaximumDemandTotal, &[1, 0, 4, 2, 0,255],"Negative reactive cumulative maximum demand (Q-) total [kvar]"),
    (ReactiveCumulativeMaximumDemandQ1Total, &[1, 0, 5, 2, 0,255],"Reactive cumulative maximum demand in Q1 (Q1) total [kvar]"),
    (ReactiveCumulativeMaximumDemandQ2Total, &[1, 0, 6, 2, 0,255],"Reactive cumulative maximum demand in Q2 (Q2) total [kvar]"),
    (ReactiveCumulativeMaximumDemandQ3Total, &[1, 0, 7, 2, 0,255],"Reactive cumulative maximum demand in Q3 (Q3) total [kvar]"),
    (ReactiveCumulativeMaximumDemandQ4Total, &[1, 0, 8, 2, 0,255],"Reactive cumulative maximum demand in Q4 (Q4) total [kvar]"),
    (ApparentCumulativeMaximumDemandTotal, &[1, 0, 9, 2, 0,255],"Apparent cumulative maximum demand (S+) total [kVA]"),
    (PositiveActiveDemandCurrentDemandPeriod, &[1, 0, 1, 4, 0,255],"Positive active demand in a current demand period (A+) [kW]"),
    (NegativeActiveDemandCurrentDemandPeriod, &[1, 0, 2, 4, 0,255],"Negative active demand in a current demand period (A-) [kW]"),
    (AbsoluteActiveDemandCurrentDemandPeriod, &[1, 0, 15, 4, 0,255],"Absolute active demand in a current demand period (|A|) [kW]"),
    (PositiveReactiveDemandCurrentDemandPeriod, &[1, 0, 3, 4, 0,255],"Positive reactive demand in a current demand period (Q+) [kvar]"),
    (NegativeReactiveDemandCurrentDemandPeriod, &[1, 0, 4, 4, 0,255],"Negative reactive demand in a current demand period (Q-) [kvar]"),
    (ReactiveDemandCurrentDemandPeriodQ1, &[1, 0, 5, 4, 0,255],"Reactive demand in a current demand period in Q1 (Q1) [kvar]"),
    (ReactiveDemandCurrentDemandPeriodQ2, &[1, 0, 6, 4, 0,255],"Reactive demand in a current demand period in Q2 (Q2) [kvar]"),
    (ReactiveDemandCurrentDemandPeriodQ3, &[1, 0, 7, 4, 0,255],"Reactive demand in a current demand period in Q3 (Q3) [kvar]"),
    (ReactiveDemandCurrentDemandPeriodQ4, &[1, 0, 8, 4, 0,255],"Reactive demand in a current demand period in Q4 (Q4) [kvar]"),
    (ApparentDemandInCurrentDemandPeriod, &[1, 0, 9, 4, 0,255],"Apparent demand in a current demand period (S+) [kVA]"),
    (PositiveActiveDemandLastCompletedDemandPeriod, &[1, 0, 1, 5, 0,255],"Positive active demand in the last completed demand period (A+) [kW]"),
    (NegativeActiveDemandLastCompletedDemandPeriod, &[1, 0, 2, 5, 0,255],"Negative active demand in the last completed demand period (A-) [kW]"),
    (AbsoluteActiveDemandLastCompletedDemandPeriod, &[1, 0, 15, 5, 0,255],"Absolute active demand in the last completed demand period (|A|) [kW]"),
    (PositiveReactiveDemandLastCompletedDemandPeriod, &[1, 0, 3, 5, 0,255],"Positive reactive demand in the last completed demand period (Q+) [kvar]"),
    (NegativeReactiveDemandLastCompletedDemandPeriod, &[1, 0, 4, 5, 0,255],"Negative reactive demand in the last completed demand period (Q-) [kvar]"),
    (ReactiveDemandLastCompletedDemandPeriodQ1, &[1, 0, 5, 5, 0,255],"Reactive demand in the last completed demand period in Q1 (Q1) [kvar]"),
    (ReactiveDemandLastCompletedDemandPeriodQ2, &[1, 0, 6, 5, 0,255],"Reactive demand in the last completed demand period in Q2 (Q2) [kvar]"),
    (ReactiveDemandLastCompletedDemandPeriodQ3, &[1, 0, 7, 5, 0,255],"Reactive demand in the last completed demand period in Q3 (Q3) [kvar]"),
    (ReactiveDemandLastCompletedDemandPeriodQ4, &[1, 0, 8, 5, 0,255],"Reactive demand in the last completed demand period in Q4 (Q4) [kvar]"),
    (ApparentDemandLastCompletedDemandPeriod, &[1, 0, 9, 5, 0,255],"Apparent demand in the last completed demand period (S+) [kVA]"),
    (PositiveActiveInstantaneousPower, &[1, 0, 1, 7, 0,255],"Positive active instantaneous power (A+) [kW]"),
    (PositiveActiveInstantaneousPowerPhaseL1, &[1, 0, 21, 7, 0,255],"Positive active instantaneous power (A+) in phase L1 [kW]"),
    (PositiveActiveInstantaneousPowerPhaseL2, &[1, 0, 41, 7, 0,255],"Positive active instantaneous power (A+) in phase L2 [kW]"),
    (PositiveActiveInstantaneousPowerPhaseL3, &[1, 0, 61, 7, 0,255],"Positive active instantaneous power (A+) in phase L3 [kW]"),
    (NegativeActiveInstantaneousPower, &[1, 0, 2, 7, 0,255],"Negative active instantaneous power (A-) [kW]"),
    (NegativeActiveInstantaneousPowerPhaseL1, &[1, 0, 22, 7, 0,255],"Negative active instantaneous power (A-) in phase L1 [kW]"),
    (NegativeActiveInstantaneousPowerPhaseL2, &[1, 0, 42, 7, 0,255],"Negative active instantaneous power (A-) in phase L2 [kW]"),
    (NegativeActiveInstantaneousPowerPhaseL3, &[1, 0, 62, 7, 0,255],"Negative active instantaneous power (A-) in phase L3 [kW]"),
    (AbsoluteActiveInstantaneousPower, &[1, 0, 15, 7, 0,255],"Absolute active instantaneous power (|A|) [kW]"),
    (AbsoluteActiveInstantaneousPowerPhaseL1, &[1, 0, 35, 7, 0,255],"Absolute active instantaneous power (|A|) in phase L1 [kW]"),
    (AbsoluteActiveInstantaneousPowerPhaseL2, &[1, 0, 55, 7, 0,255],"Absolute active instantaneous power (|A|) in phase L2 [kW]"),
    (AbsoluteActiveInstantaneousPowerPhaseL3, &[1, 0, 75, 7, 0,255],"Absolute active instantaneous power (|A|) in phase L3 [kW]"),
    (SumActiveInstantaneousPower, &[1, 0, 16, 7, 0,255],"Sum active instantaneous power (A+ - A-) [kW]"),
    (SumActiveInstantaneousPowerPhaseL1, &[1, 0, 36, 7, 0,255],"Sum active instantaneous power (A+ - A-) in phase L1 [kW]"),
    (SumActiveInstantaneousPowerPhaseL2, &[1, 0, 56, 7, 0,255],"Sum active instantaneous power (A+ - A-) in phase L2 [kW]"),
    (SumActiveInstantaneousPowerPhaseL3, &[1, 0, 76, 7, 0,255],"Sum active instantaneous power (A+ - A-) in phase L3 [kW]"),
    (PositiveReactiveInstantaneousPower, &[1, 0, 3, 7, 0,255],"Positive reactive instantaneous power (Q+) [kvar]"),
    (PositiveReactiveInstantaneousPowerPhaseL1, &[1, 0, 23, 7, 0,255],"Positive reactive instantaneous power (Q+) in phase L1 [kvar]"),
    (PositiveReactiveInstantaneousPowerPhaseL2, &[1, 0, 43, 7, 0,255],"Positive reactive instantaneous power (Q+) in phase L2 [kvar]"),
    (PositiveReactiveInstantaneousPowerPhaseL3, &[1, 0, 63, 7, 0,255],"Positive reactive instantaneous power (Q+) in phase L3 [kvar]"),
    (NegativeReactiveInstantaneousPower, &[1, 0, 4, 7, 0,255],"Negative reactive instantaneous power (Q-) [kvar]"),
    (NegativeReactiveInstantaneousPowerPhaseL1, &[1, 0, 24, 7, 0,255],"Negative reactive instantaneous power (Q-) in phase L1 [kvar]"),
    (NegativeReactiveInstantaneousPowerPhaseL2, &[1, 0, 44, 7, 0,255],"Negative reactive instantaneous power (Q-) in phase L2 [kvar]"),
    (NegativeReactiveInstantaneousPowerPhaseL3, &[1, 0, 64, 7, 0,255],"Negative reactive instantaneous power (Q-) in phase L3 [kvar]"),
    (ApparentInstantaneousPower, &[1, 0, 9, 7, 0,255],"Apparent instantaneous power (S+) [kVA]"),
    (ApparentInstantaneousPowerPhaseL1, &[1, 0, 29, 7, 0,255],"Apparent instantaneous power (S+) in phase L1 [kVA]"),
    (ApparentInstantaneousPowerPhaseL2, &[1, 0, 49, 7, 0,255],"Apparent instantaneous power (S+) in phase L2 [kVA]"),
    (ApparentInstantaneousPowerPhaseL3, &[1, 0, 69, 7, 0,255],"Apparent instantaneous power (S+) in phase L3 [kVA]"),
    (InstantaneousCurrent, &[1, 0, 11, 7, 0,255],"Instantaneous current (I) [A]"),
    (InstantaneousCurrentPhaseL1, &[1, 0, 31, 7, 0,255],"Instantaneous current (I) in phase L1 [A]"),
    (InstantaneousCurrentPhaseL2, &[1, 0, 51, 7, 0,255],"Instantaneous current (I) in phase L2 [A]"),
    (InstantaneousCurrentPhaseL3, &[1, 0, 71, 7, 0,255],"Instantaneous current (I) in phase L3 [A]"),
    (InstantaneousCurrentNeutral, &[1, 0, 91, 7, 0,255],"Instantaneous current (I) in neutral [A]"),
    (MaximumCurrent, &[1, 0, 11, 6, 0,255],"Maximum current (I max) [A]"),
    (MaximumCurrentPhaseL1, &[1, 0, 31, 6, 0,255],"Maximum current (I max) in phase L1 [A]"),
    (MaximumCurrentPhaseL2, &[1, 0, 51, 6, 0,255],"Maximum current (I max) in phase L2 [A]"),
    (MaximumCurrentPhaseL3, &[1, 0, 71, 6, 0,255],"Maximum current (I max) in phase L3 [A]"),
    (MaximumCurrentNeutral, &[1, 0, 91, 6, 0,255],"Maximum current (I max) in neutral [A]"),
    (InstantaneousVoltage, &[1, 0, 12, 7, 0,255],"Instantaneous voltage (U) [V]"),
    (InstantaneousVoltagePhaseL1, &[1, 0, 32, 7, 0,255],"Instantaneous voltage (U) in phase L1 [V]"),
    (InstantaneousVoltagePhaseL2, &[1, 0, 52, 7, 0,255],"Instantaneous voltage (U) in phase L2 [V]"),
    (InstantaneousVoltagePhaseL3, &[1, 0, 72, 7, 0,255],"Instantaneous voltage (U) in phase L3 [V]"),
    (InstantaneousPowerFactor, &[1, 0, 13, 7, 0,255],"Instantaneous power factor"),
    (InstantaneousPowerFactorPhaseL1, &[1, 0, 33, 7, 0,255],"Instantaneous power factor in phase L1"),
    (InstantaneousPowerFactorPhaseL2, &[1, 0, 53, 7, 0,255],"Instantaneous power factor in phase L2"),
    (InstantaneousPowerFactorPhaseL3, &[1, 0, 73, 7, 0,255],"Instantaneous power factor in phase L3"),
    (Frequency, &[1, 0, 14, 7, 0,255],"Frequency [Hz]")
}
