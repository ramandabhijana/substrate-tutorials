use crate::{self as pallet_to_mock, PriceOracle};
use frame_support::{construct_runtime, derive_impl, parameter_types, traits::Currency};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

construct_runtime!(
	pub enum TestRuntime
	{
		System: frame_system,
		Balances: pallet_balances,
		PalletToMock: pallet_to_mock,
	}
);

type Block = frame_system::mocking::MockBlock<TestRuntime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for TestRuntime {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockLength = ();
	type BlockWeights = ();
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type RuntimeEvent = RuntimeEvent;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type RuntimeOrigin = RuntimeOrigin;
	type PalletInfo = PalletInfo;
	type SystemWeightInfo = ();
	type Version = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
	pub const MaxFreezes: u32 = 100;
}

impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = MaxFreezes;
}

parameter_types! {
	pub const ValueToMint: u64 = 200;
}

pub struct MockPriceOracle;
impl PriceOracle for MockPriceOracle {
	type Error = ();

	fn get_price() -> Result<u64, Self::Error> {
		Ok(100)
	}
}

impl pallet_to_mock::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ValueToMint = ValueToMint;
	type SomePriceOracle = MockPriceOracle;
}

// Mock users AccountId
pub const ALICE: u64 = 1;

#[derive(Default)]
pub struct ExtBuilder {
	caps_endowed_accounts: Vec<(u64, u64)>,
}

impl ExtBuilder {
	pub fn balances(mut self, accounts: Vec<(u64, u64)>) -> Self {
		for account in accounts {
			self.caps_endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap();

		pallet_balances::GenesisConfig::<TestRuntime> {
			balances: self.caps_endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	ExtBuilder::default().build()
}
