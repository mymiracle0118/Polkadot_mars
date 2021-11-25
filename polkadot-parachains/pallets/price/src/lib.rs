#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_support::sp_std::vec;
    use frame_support::inherent::Vec;
	use frame_system::pallet_prelude::*;
    use xcm::v0::{ Junction, OriginKind, SendXcm, Xcm,};
	use cumulus_primitives_core::ParaId;
	use cumulus_pallet_xcm::{Origin as CumulusOrigin, ensure_sibling_para};
	use frame_support::sp_runtime::print;
	//use ares_oracle::getPriceBySymbol;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    pub struct ResPriceCall<AccountId> {
		call_index: [u8; 2],
		account: AccountId,
		price: u32,
	}

	impl<AccountId> ResPriceCall<AccountId> {
		pub fn new(pallet_index: u8, method_index: u8, account: AccountId, price: u32,) 
		-> Self {
			ResPriceCall {
				call_index: [pallet_index, method_index],
				account: account,
				price: price,
			}
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + ares_oracle::Config
		where sp_runtime::AccountId32: From<<Self as frame_system::Config>::AccountId>
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// origin could from both extrinsic and xcm message
		type Origin: From<<Self as frame_system::Config>::Origin> + Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		type Call: From<Call<Self>> + Encode;
		//type Call: From<Call<Self>>;
        /// The XCM sender module.
		type XcmSender: SendXcm;
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> 
		where sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>
	{
		GetPriceFinish(ParaId, T::AccountId, Vec<u8>),
        SendPriceSuccess(ParaId, T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
        BridgeXcmSendError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> 
		where sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>
	{}

	#[pallet::call]
	impl<T: Config> Pallet<T> 
		where sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
	{
		#[pallet::weight(0)]
		pub fn price(origin: OriginFor<T>, account: T::AccountId, symbol: Vec<u8>, ori_pallet_idx: u8, ori_method_idx: u8) -> DispatchResultWithPostInfo {
			print("enter bridge pallet_idx ");
            log::info!("account {:?} pallet_idx {:?} method_idx {:?}", account.clone(), ori_pallet_idx, ori_method_idx);
			
			let para_id = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
            log::info!("para_id {:?}", para_id);
            
			let price = ares_oracle::Pallet::<T>::get_price_symbol(symbol.clone());

			log::info!("get price from ares:{:?}", price);
			// emit event
			Self::deposit_event(Event::GetPriceFinish(para_id, account.clone(), symbol.clone()));
            
            let call = ResPriceCall::<T::AccountId>::new(
				ori_pallet_idx, 
				ori_method_idx,
				account.clone(),
				price);
	
			// build the xcm transact message
			let message = Xcm::Transact { 
				origin_type: OriginKind::Native, 
				require_weight_at_most: 0, 
				call: call.encode().into() };

            log::info!("before send bridge xcm");
            match T::XcmSender::send_xcm(
                (Junction::Parent, Junction::Parachain(para_id.into())).into(), 
					message) {
                Ok(()) => { 
                    Self::deposit_event(Event::SendPriceSuccess(para_id, account.clone(), symbol));
                    Ok(().into())
                },
                Err(e) => {
					log::info!("bridge xcm send error ******** {:?}", e);
					//print(e);
					//Self::deposit_event(Event::SomeError(who, e.));
					Err(Error::<T>::BridgeXcmSendError.into())
					//Err(e.into())
				},
			}
		}
	}
}