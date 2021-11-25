use super::*;
use pallet_vesting;

parameter_types! {
    pub const MinVestedTransfer: Balance = 1 * AMAS_CENTS; 
}

impl pallet_vesting::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
}