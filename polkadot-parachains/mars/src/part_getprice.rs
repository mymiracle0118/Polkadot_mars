use super::*;
use pallet_getprice;

parameter_types! {
	pub const PricePalletID: u8 = 100;
	pub const PriceMethodID: u8 = 0;
	pub const PriceWeightAtMost: u64 = 1000;
}

impl pallet_getprice::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Call = Call;
	type XcmSender = XcmRouter;
	type PricePalletID = PricePalletID;
	type PriceMethodID = PriceMethodID;
	type PriceWeightAtMost = PriceWeightAtMost;
}