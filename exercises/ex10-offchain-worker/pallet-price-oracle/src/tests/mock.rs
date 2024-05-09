use crate as pallet_price_oracle;
use frame_support::{derive_impl, parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, ConstU32, ConstU64, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;
pub type Extrinsic = TestXt<RuntimeCall, ()>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime
	{
		System: frame_system,
		PriceOracle: pallet_price_oracle,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for TestRuntime {
	type AccountData = ();
	type AccountId = sp_core::sr25519::Public;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = ConstU64<250>;
	type BlockLength = ();
	type BlockWeights = ();
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type RuntimeEvent = RuntimeEvent;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type MaxConsumers = ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type RuntimeOrigin = RuntimeOrigin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_price_oracle::Config for TestRuntime {
	type Call = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for TestRuntime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = Extrinsic;
	type OverarchingCall = RuntimeCall;
}
