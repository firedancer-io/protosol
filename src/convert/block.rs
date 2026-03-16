use crate::protos;

impl From<protos::Inflation> for solana_inflation::Inflation {
    fn from(input: protos::Inflation) -> Self {
        let mut inflation = solana_inflation::Inflation::default();
        inflation.initial = input.initial;
        inflation.terminal = input.terminal;
        inflation.taper = input.taper;
        inflation.foundation = input.foundation;
        inflation.foundation_term = input.foundation_term;
        inflation
    }
}
