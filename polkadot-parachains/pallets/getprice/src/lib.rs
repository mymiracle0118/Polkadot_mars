#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_support::inherent::Vec;
	use frame_system::pallet_prelude::*;
	use xcm::v0::{ Junction, OriginKind, SendXcm, Xcm,};
	use cumulus_pallet_xcm::{Origin as CumulusOrigin};
	use cumulus_primitives_core::ParaId;
	use frame_support::sp_runtime::print;
	
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
	pub struct GetPriceCall<AccountId> {
		call_index: [u8; 2],
		account: AccountId,
		symbol: Vec<u8>,
		my_pallet_idx: u8,
		my_method_idx: u8,
	}

	impl<AccountId> GetPriceCall<AccountId> {
		pub fn new(pallet_index: u8, method_index: u8, account: AccountId, symbol: Vec<u8>,) 
		-> Self {
			GetPriceCall {
				call_index: [pallet_index, method_index],
				account: account,
				symbol: symbol,
				my_pallet_idx: 101,
				my_method_idx: 1,
			}
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as frame_system::Config>::Origin> + Into<Result<CumulusOrigin, <Self as Config>::Origin>>;
		
		type Call: From<Call<Self>> + Encode;
		/// The XCM sender module.
		type XcmSender: SendXcm;

		type BridgePalletID: Get<u8>;

		type BridgeMethodID: Get<u8>;

		type BridgeWeightAtMost: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GetPriceSend(T::AccountId, Vec<u8>),
        GetPriced(T::AccountId, Vec<u8>),
		//SomeError(T::AccountId, Error),
	}

	#[pallet::error]
	pub enum Error<T> {
		XcmSendError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn getprice(origin: OriginFor<T>, para: ParaId, symbol: Vec<u8>) -> DispatchResultWithPostInfo {
			print("ent getprice");
			let who = ensure_signed(origin)?;

			// compose the call with pallet id, method id and arguments
			let call = GetPriceCall::<T::AccountId>::new(
				T::BridgePalletID::get(), 
				T::BridgeMethodID::get(),
				who.clone(),
				symbol.clone());
	
			// build the xcm transact message
			let message = Xcm::Transact { 
				origin_type: OriginKind::Native, 
				require_weight_at_most: T::BridgeWeightAtMost::get(), 
				call: call.encode().into() };
			
			// send the message to xregister server chain
			match T::XcmSender::send_xcm((Junction::Parent, 
					Junction::Parachain(para.into())).into(), 
					message) {
				Ok(()) => {
					// emit the event if send successfully
					Self::deposit_event(Event::GetPriceSend(who, symbol));
					Ok(().into())
				},
				Err(e) => {
					log::info!("xcm send error ******** {:?}", e);
					//print(e);
					//Self::deposit_event(Event::SomeError(who, e.));
					Err(Error::<T>::XcmSendError.into())
					//Err(e.into())
				}
			}
		}

        #[pallet::weight(0)]
		pub fn res_price(origin: OriginFor<T>, account: T::AccountId, price: Vec<u8>) -> DispatchResultWithPostInfo {
			print("res_price");
			// Only accept pings from other chains.
			//let who = ensure_signed(origin.clone())?;

			Self::deposit_event(Event::GetPriced(account, price));
			print("get price event finish");
			
			Ok(().into())
		}
	}
}