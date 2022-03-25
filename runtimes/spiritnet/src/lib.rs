// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! The KILT runtime. This can be compiled with `#[no_std]`, ready for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{Contains, EnsureOneOf, InstanceFilter, PrivilegeCmp},
	weights::{constants::RocksDbWeight, Weight},
};
use frame_system::EnsureRoot;
use sp_api::impl_runtime_apis;
use sp_core::{
	u32_trait::{_1, _2, _3, _5},
	OpaqueMetadata,
};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, OpaqueKeys, Verify},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Perbill, Permill, Perquintill, RuntimeDebug,
};
use sp_std::{cmp::Ordering, prelude::*};
use sp_version::RuntimeVersion;

pub use parachain_staking::InflationInfo;
use runtime_common::{
	constants::{self, KILT, MICRO_KILT, MILLI_KILT},
	fees::{ToAuthor, WeightToFee},
	pallet_id, AccountId, AuthorityId, Balance, BlockHashCount, BlockLength, BlockNumber, BlockWeights, DidIdentifier,
	FeeSplit, Hash, Header, Index, Signature, SlowAdjustingFeeUpdate,
};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
#[cfg(feature = "runtime-benchmarks")]
use {frame_system::EnsureSigned, kilt_support::signature::AlwaysVerify, runtime_common::benchmarks::DummySignature};

#[cfg(test)]
mod tests;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

mod weights;

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("kilt-spiritnet"),
	impl_name: create_runtime_str!("kilt-spiritnet"),
	authoring_version: 1,
	spec_version: 10600,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled
/// natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const SS58Prefix: u8 = 38;
}

pub struct BaseFilter;
impl Contains<Call> for BaseFilter {
	fn contains(c: &Call) -> bool {
		!matches!(
			c,
			Call::KiltLaunch(kilt_launch::Call::locked_transfer { .. }) | Call::Delegation { .. }
		)
	}
}

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in
	/// dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = runtime_common::Header;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest
	/// pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = RocksDbWeight;
	type BaseCallFilter = BaseFilter;
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type SS58Prefix = SS58Prefix;
	/// The set code logic
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Runtime>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = constants::SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 10 * MILLI_KILT;
	pub const TransactionByteFee: u128 = MICRO_KILT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = Index;
	type Currency = pallet_balances::Pallet<Runtime>;
	type Deposit = constants::IndicesDeposit;
	type Event = Event;
	type WeightInfo = weights::pallet_indices::WeightInfo<Runtime>;
}

impl pallet_balances::Config for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = Treasury;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	/// This value increases the priority of `Operational` transactions by adding
	/// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, FeeSplit<Runtime, Treasury, ToAuthor<Runtime>>>;
	type TransactionByteFee = TransactionByteFee;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee<Runtime>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = constants::MAXIMUM_BLOCK_WEIGHT / 4;
	pub const ReservedDmpWeight: Weight = constants::MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type Event = Event;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = ();
	type DmpMessageHandler = ();
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = ();
	type ReservedXcmpWeight = ReservedXcmpWeight;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	pub const MaxAuthorities: u32 = constants::staking::MAX_CANDIDATES;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuthorityId;
	//TODO: handle disabled validators
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ParachainStaking;
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = ParachainStaking;
	type NextSessionRotation = ParachainStaking;
	type SessionManager = ParachainStaking;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = constants::MIN_VESTED_TRANSFER_AMOUNT;
}

impl pallet_vesting::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	// disable vested transfers by setting min amount to max balance
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = weights::pallet_vesting::WeightInfo<Runtime>;
	const MAX_VESTING_SCHEDULES: u32 = runtime_common::constants::MAX_VESTING_SCHEDULES;
}

parameter_types! {
	pub const MaxClaims: u32 = 50;
	pub const UsableBalance: Balance = KILT;
	pub const AutoUnlockBound: u32 = 100;
}

impl kilt_launch::Config for Runtime {
	type Event = Event;
	type MaxClaims = MaxClaims;
	type UsableBalance = UsableBalance;
	type AutoUnlockBound = AutoUnlockBound;
	type WeightInfo = weights::kilt_launch::WeightInfo<Runtime>;
	type PalletId = pallet_id::Launch;
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxSize = constants::preimage::PreimageMaxSize;
	type BaseDeposit = constants::preimage::PreimageBaseDeposit;
	type ByteDeposit = constants::ByteDeposit;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub const NoPreimagePostponement: Option<BlockNumber> = Some(10);
}

type ScheduleOrigin = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>,
>;

/// Used the compare the privilege of an origin inside the scheduler.
pub struct OriginPrivilegeCmp;

impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
	fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
		if left == right {
			return Some(Ordering::Equal);
		}

		match (left, right) {
			// Root is greater than anything.
			(OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
			// Check which one has more yes votes.
			(
				OriginCaller::Council(pallet_collective::RawOrigin::Members(l_yes_votes, l_count)),
				OriginCaller::Council(pallet_collective::RawOrigin::Members(r_yes_votes, r_count)),
			) => Some((l_yes_votes * r_count).cmp(&(r_yes_votes * l_count))),
			// For every other origin we don't care, as they are not used for `ScheduleOrigin`.
			_ => None,
		}
	}
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = ScheduleOrigin;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
	type OriginPrivilegeCmp = OriginPrivilegeCmp;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = constants::governance::LAUNCH_PERIOD;
	pub const VotingPeriod: BlockNumber = constants::governance::VOTING_PERIOD;
	pub const FastTrackVotingPeriod: BlockNumber = constants::governance::FAST_TRACK_VOTING_PERIOD;
	pub const MinimumDeposit: Balance = constants::governance::MIN_DEPOSIT;
	pub const EnactmentPeriod: BlockNumber = constants::governance::ENACTMENT_PERIOD;
	pub const CooloffPeriod: BlockNumber = constants::governance::COOLOFF_PERIOD;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type VoteLockingPeriod = VotingPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
	/// A majority can have the next scheduled referendum be a straight
	/// majority-carries vote.
	type ExternalMajorityOrigin = pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
	/// A unanimous council can have the next scheduled referendum be a straight
	/// default-carries (NTB) vote.
	type ExternalDefaultOrigin = pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, CouncilCollective>;
	/// Two thirds of the technical committee can have an
	/// ExternalMajority/ExternalDefault vote be tabled immediately and with a
	/// shorter voting/enactment period.
	type FastTrackOrigin = pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalCollective>;
	type InstantOrigin = pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>;
	type InstantAllowed = InstantAllowed;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to
	// it.
	type CancellationOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilCollective>,
	>;
	// To cancel a proposal before it has been passed, the technical committee must
	// be unanimous or Root must agree.
	type CancelProposalOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>,
	>;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Any single technical committee member may veto a coming council proposal,
	// however they can only do it once and it lasts only for the cooloff period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = constants::ByteDeposit;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = MaxVotes;
	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
	type MaxProposals = MaxProposals;

	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 20 * KILT;
	pub const SpendPeriod: BlockNumber = constants::governance::SPEND_PERIOD;
	pub const Burn: Permill = Permill::zero();
	pub const MaxApprovals: u32 = 100;
}

type ApproveOrigin = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilCollective>,
>;

type MoreThanHalfCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>,
>;

impl pallet_treasury::Config for Runtime {
	type PalletId = pallet_id::Treasury;
	type Currency = Balances;
	type ApproveOrigin = ApproveOrigin;
	type RejectOrigin = MoreThanHalfCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
	type MaxApprovals = MaxApprovals;
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = constants::governance::COUNCIL_MOTION_DURATION;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = constants::governance::TECHNICAL_MOTION_DURATION;
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;
}

type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = TechnicalMaxProposals;
	type MaxMembers = TechnicalMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

impl pallet_membership::Config for Runtime {
	type Event = Event;
	type AddOrigin = MoreThanHalfCouncil;
	type RemoveOrigin = MoreThanHalfCouncil;
	type SwapOrigin = MoreThanHalfCouncil;
	type ResetOrigin = MoreThanHalfCouncil;
	type PrimeOrigin = MoreThanHalfCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = TechnicalMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MaxDelegatedAttestations: u32 = 1000;
	pub const AttestationDeposit: Balance = constants::attestation::ATTESTATION_DEPOSIT;
}

impl attestation::Config for Runtime {
	type EnsureOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;

	type Event = Event;
	type WeightInfo = weights::attestation::WeightInfo<Runtime>;

	type Currency = Balances;
	type Deposit = AttestationDeposit;
	type MaxDelegatedAttestations = MaxDelegatedAttestations;
}

parameter_types! {
	pub const MaxSignatureByteLength: u16 = constants::delegation::MAX_SIGNATURE_BYTE_LENGTH;
	pub const MaxParentChecks: u32 = constants::delegation::MAX_PARENT_CHECKS;
	pub const MaxRevocations: u32 = constants::delegation::MAX_REVOCATIONS;
	pub const MaxRemovals: u32 = constants::delegation::MAX_REMOVALS;
	#[derive(Clone)]
	pub const MaxChildren: u32 = constants::delegation::MAX_CHILDREN;
	pub const DelegationDeposit: Balance = constants::delegation::DELEGATION_DEPOSIT;
}

impl delegation::Config for Runtime {
	type DelegationEntityId = DidIdentifier;
	type DelegationNodeId = Hash;

	type EnsureOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type DelegationSignatureVerification = did::DidSignatureVerify<Runtime>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type Signature = did::DidSignature;

	#[cfg(feature = "runtime-benchmarks")]
	type Signature = DummySignature;
	#[cfg(feature = "runtime-benchmarks")]
	type DelegationSignatureVerification = AlwaysVerify<AccountId, Vec<u8>, Self::Signature>;

	type Event = Event;
	type MaxSignatureByteLength = MaxSignatureByteLength;
	type MaxParentChecks = MaxParentChecks;
	type MaxRevocations = MaxRevocations;
	type MaxRemovals = MaxRemovals;
	type MaxChildren = MaxChildren;
	type WeightInfo = weights::delegation::WeightInfo<Runtime>;
	type Currency = Balances;
	type Deposit = DelegationDeposit;
}

parameter_types! {
	pub const Fee: Balance = MILLI_KILT;
}

impl ctype::Config for Runtime {
	type CtypeCreatorId = AccountId;
	type Currency = Balances;
	type Fee = Fee;
	type FeeCollector = Treasury;

	type EnsureOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;

	type Event = Event;
	type WeightInfo = weights::ctype::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MaxNewKeyAgreementKeys: u32 = constants::did::MAX_KEY_AGREEMENT_KEYS;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxUrlLength: u32 = constants::did::MAX_URL_LENGTH;
	pub const MaxPublicKeysPerDid: u32 = constants::did::MAX_PUBLIC_KEYS_PER_DID;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxTotalKeyAgreementKeys: u32 = constants::did::MAX_TOTAL_KEY_AGREEMENT_KEYS;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxEndpointUrlsCount: u32 = constants::did::MAX_ENDPOINT_URLS_COUNT;
	// Standalone block time is half the duration of a parachain block.
	pub const MaxBlocksTxValidity: BlockNumber = constants::did::MAX_BLOCKS_TX_VALIDITY;
	pub const DidDeposit: Balance = constants::did::DID_DEPOSIT;
	pub const DidFee: Balance = constants::did::DID_FEE;
	pub const MaxNumberOfServicesPerDid: u32 = constants::did::MAX_NUMBER_OF_SERVICES_PER_DID;
	pub const MaxServiceIdLength: u32 = constants::did::MAX_SERVICE_ID_LENGTH;
	pub const MaxServiceTypeLength: u32 = constants::did::MAX_SERVICE_TYPE_LENGTH;
	pub const MaxServiceUrlLength: u32 = constants::did::MAX_SERVICE_URL_LENGTH;
	pub const MaxNumberOfTypesPerService: u32 = constants::did::MAX_NUMBER_OF_TYPES_PER_SERVICE;
	pub const MaxNumberOfUrlsPerService: u32 = constants::did::MAX_NUMBER_OF_URLS_PER_SERVICE;
}

impl did::Config for Runtime {
	type DidIdentifier = DidIdentifier;
	type Event = Event;
	type Call = Call;
	type Origin = Origin;
	type Currency = Balances;
	type Deposit = DidDeposit;
	type Fee = DidFee;
	type FeeCollector = Treasury;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type EnsureOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;

	#[cfg(feature = "runtime-benchmarks")]
	type EnsureOrigin = EnsureSigned<DidIdentifier>;
	#[cfg(feature = "runtime-benchmarks")]
	type OriginSuccess = DidIdentifier;

	type MaxNewKeyAgreementKeys = MaxNewKeyAgreementKeys;
	type MaxTotalKeyAgreementKeys = MaxTotalKeyAgreementKeys;
	type MaxPublicKeysPerDid = MaxPublicKeysPerDid;
	type MaxBlocksTxValidity = MaxBlocksTxValidity;
	type MaxNumberOfServicesPerDid = MaxNumberOfServicesPerDid;
	type MaxServiceIdLength = MaxServiceIdLength;
	type MaxServiceTypeLength = MaxServiceTypeLength;
	type MaxServiceUrlLength = MaxServiceUrlLength;
	type MaxNumberOfTypesPerService = MaxNumberOfTypesPerService;
	type MaxNumberOfUrlsPerService = MaxNumberOfUrlsPerService;
	type WeightInfo = weights::did::WeightInfo<Runtime>;
}

parameter_types! {
	pub const DidLookupDeposit: Balance = constants::did_lookup::DID_CONNECTION_DEPOSIT;
}

impl pallet_did_lookup::Config for Runtime {
	type Event = Event;
	type Signature = Signature;
	type Signer = <Signature as Verify>::Signer;
	type DidIdentifier = DidIdentifier;

	type Currency = Balances;
	type Deposit = DidLookupDeposit;

	type EnsureOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;

	type WeightInfo = weights::pallet_did_lookup::WeightInfo<Runtime>;
}

parameter_types! {
	pub const Web3NameDeposit: Balance = constants::web3_names::DEPOSIT;
	pub const MinNameLength: u32 = constants::web3_names::MIN_LENGTH;
	pub const MaxNameLength: u32 = constants::web3_names::MAX_LENGTH;
}

impl pallet_web3_names::Config for Runtime {
	type BanOrigin = EnsureRoot<AccountId>;
	type OwnerOrigin = did::EnsureDidOrigin<DidIdentifier, AccountId>;
	type OriginSuccess = did::DidRawOrigin<AccountId, DidIdentifier>;
	type Currency = Balances;
	type Deposit = Web3NameDeposit;
	type Event = Event;
	type MaxNameLength = MaxNameLength;
	type MinNameLength = MinNameLength;
	type Web3Name = pallet_web3_names::web3_name::AsciiWeb3Name<Runtime, MinNameLength, MaxNameLength>;
	type Web3NameOwner = DidIdentifier;
	type WeightInfo = weights::pallet_web3_names::WeightInfo<Runtime>;
}

parameter_types! {
	pub const InitialPeriodLength: BlockNumber = constants::treasury::INITIAL_PERIOD_LENGTH;
	pub const InitialPeriodReward: Balance = constants::treasury::INITIAL_PERIOD_REWARD_PER_BLOCK;
}

impl pallet_inflation::Config for Runtime {
	type Currency = Balances;
	type InitialPeriodLength = InitialPeriodLength;
	type InitialPeriodReward = InitialPeriodReward;
	type Beneficiary = Treasury;
	type WeightInfo = weights::pallet_inflation::WeightInfo<Runtime>;
}

parameter_types! {
	/// Minimum round length is 1 hour
	pub const MinBlocksPerRound: BlockNumber = constants::staking::MIN_BLOCKS_PER_ROUND;
	/// Default length of a round/session is 2 hours
	pub const DefaultBlocksPerRound: BlockNumber = constants::staking::DEFAULT_BLOCKS_PER_ROUND;
	/// Unstaked balance can be unlocked after 7 days
	pub const StakeDuration: BlockNumber = constants::staking::STAKE_DURATION;
	/// Collator exit requests are delayed by 4 hours (2 rounds/sessions)
	pub const ExitQueueDelay: u32 = 2;
	/// Minimum 16 collators selected per round, default at genesis and minimum forever after
	pub const MinCollators: u32 = constants::staking::MIN_COLLATORS;
	/// At least 4 candidates which cannot leave the network if there are no other candidates.
	pub const MinRequiredCollators: u32 = 4;
	/// We only allow one delegation per round.
	pub const MaxDelegationsPerRound: u32 = 1;
	/// Maximum 25 delegators per collator at launch, might be increased later
	#[derive(Debug, PartialEq)]
	pub const MaxDelegatorsPerCollator: u32 = constants::staking::MAX_DELEGATORS_PER_COLLATOR;
	/// Maximum 1 collator per delegator at launch, will be increased later
	#[derive(Debug, PartialEq)]
	pub const MaxCollatorsPerDelegator: u32 = 1;
	/// Minimum stake required to be reserved to be a collator is 10_000
	pub const MinCollatorStake: Balance = 10_000 * KILT;
	/// Minimum stake required to be reserved to be a delegator is 1000
	pub const MinDelegatorStake: Balance = constants::staking::MIN_DELEGATOR_STAKE;
	/// Maximum number of collator candidates
	#[derive(Debug, PartialEq)]
	pub const MaxCollatorCandidates: u32 = constants::staking::MAX_CANDIDATES;
	/// Maximum number of concurrent requests to unlock unstaked balance
	pub const MaxUnstakeRequests: u32 = 10;
	/// The starting block number for the network rewards
	pub const NetworkRewardStart: BlockNumber = constants::treasury::INITIAL_PERIOD_LENGTH;
	/// The rate in percent for the network rewards
	pub const NetworkRewardRate: Perquintill = constants::staking::NETWORK_REWARD_RATE;
}

impl parachain_staking::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type MinBlocksPerRound = MinBlocksPerRound;
	type DefaultBlocksPerRound = DefaultBlocksPerRound;
	type StakeDuration = StakeDuration;
	type ExitQueueDelay = ExitQueueDelay;
	type MinCollators = MinCollators;
	type MinRequiredCollators = MinRequiredCollators;
	type MaxDelegationsPerRound = MaxDelegationsPerRound;
	type MaxDelegatorsPerCollator = MaxDelegatorsPerCollator;
	type MaxCollatorsPerDelegator = MaxCollatorsPerDelegator;
	type MinCollatorStake = MinCollatorStake;
	type MinCollatorCandidateStake = MinCollatorStake;
	type MaxTopCandidates = MaxCollatorCandidates;
	type MinDelegation = MinDelegatorStake;
	type MinDelegatorStake = MinDelegatorStake;
	type MaxUnstakeRequests = MaxUnstakeRequests;
	type NetworkRewardRate = NetworkRewardRate;
	type NetworkRewardStart = NetworkRewardStart;
	type NetworkRewardBeneficiary = Treasury;
	type WeightInfo = weights::parachain_staking::WeightInfo<Runtime>;
}

impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum ProxyType {
	/// Allow for any call.
	Any,
	/// Allow for calls that do not move tokens out of the caller's account.
	NonTransfer,
	/// Allow for governance-related calls.
	Governance,
	/// Allow for staking-related calls.
	ParachainStaking,
	/// Allow for calls that cancel proxy information.
	CancelProxy,
	/// Allow for calls that do not result in a deposit being claimed (e.g., for
	/// attestations, delegations, or DIDs).
	NonDepositClaiming,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => matches!(
				c,
				Call::Attestation(..)
					| Call::Authorship(..)
					// Excludes `Balances`
					| Call::Council(..) | Call::Ctype(..)
					| Call::Delegation(..)
					| Call::Democracy(..)
					| Call::Did(..)
					| Call::DidLookup(..)
					| Call::Indices(
						// Excludes `force_transfer`, and `transfer`
						pallet_indices::Call::claim { .. }
							| pallet_indices::Call::free { .. }
							| pallet_indices::Call::freeze { .. }
					)
					// Excludes `KiltLaunch`
					| Call::ParachainStaking(..)
					// Excludes `ParachainSystem`
					| Call::Preimage(..)
					| Call::Proxy(..)
					| Call::Scheduler(..)
					| Call::Session(..)
					| Call::System(..)
					| Call::TechnicalCommittee(..)
					| Call::TechnicalMembership(..)
					| Call::Timestamp(..)
					| Call::Treasury(..)
					| Call::Utility(..)
					| Call::Vesting(
						// Excludes `force_vested_transfer`, `merge_schedules`, and `vested_transfer`
						pallet_vesting::Call::vest { .. }
							| pallet_vesting::Call::vest_other { .. }
					)
					| Call::Web3Names(..),
			),
			ProxyType::NonDepositClaiming => matches!(
				c,
				Call::Attestation(
						// Excludes `reclaim_deposit`
						attestation::Call::add { .. }
							| attestation::Call::remove { .. }
							| attestation::Call::revoke { .. }
					)
					| Call::Authorship(..)
					// Excludes `Balances`
					| Call::Council(..) | Call::Ctype(..)
					| Call::Delegation(
						// Excludes `reclaim_deposit`
						delegation::Call::add_delegation { .. }
							| delegation::Call::create_hierarchy { .. }
							| delegation::Call::remove_delegation { .. }
							| delegation::Call::revoke_delegation { .. }
					)
					| Call::Democracy(..)
					| Call::Did(
						// Excludes `reclaim_deposit`
						did::Call::add_key_agreement_key { .. }
							| did::Call::add_service_endpoint { .. }
							| did::Call::create { .. }
							| did::Call::delete { .. }
							| did::Call::remove_attestation_key { .. }
							| did::Call::remove_delegation_key { .. }
							| did::Call::remove_key_agreement_key { .. }
							| did::Call::remove_service_endpoint { .. }
							| did::Call::set_attestation_key { .. }
							| did::Call::set_authentication_key { .. }
							| did::Call::set_delegation_key { .. }
							| did::Call::submit_did_call { .. }
					)
					| Call::DidLookup(
						// Excludes `reclaim_deposit`
						pallet_did_lookup::Call::associate_account { .. }
							| pallet_did_lookup::Call::associate_sender { .. }
							| pallet_did_lookup::Call::remove_account_association { .. }
							| pallet_did_lookup::Call::remove_sender_association { .. }
					)
					| Call::Indices(..)
					// Excludes `KiltLaunch`
					| Call::ParachainStaking(..)
					// Excludes `ParachainSystem`
					| Call::Preimage(..)
					| Call::Proxy(..)
					| Call::Scheduler(..)
					| Call::Session(..)
					| Call::System(..)
					| Call::TechnicalCommittee(..)
					| Call::TechnicalMembership(..)
					| Call::Timestamp(..)
					| Call::Treasury(..)
					| Call::Utility(..)
					| Call::Vesting(..)
					| Call::Web3Names(
						// Excludes `ban`, and `reclaim_deposit`
						pallet_web3_names::Call::claim { .. }
							| pallet_web3_names::Call::release_by_owner { .. }
							| pallet_web3_names::Call::unban { .. }
					),
			),
			ProxyType::Governance => matches!(
				c,
				Call::Council(..)
					| Call::Democracy(..)
					| Call::TechnicalCommittee(..)
					| Call::TechnicalMembership(..)
					| Call::Treasury(..) | Call::Utility(..)
			),
			ProxyType::ParachainStaking => {
				matches!(c, Call::ParachainStaking(..) | Call::Session(..) | Call::Utility(..))
			}
			ProxyType::CancelProxy => matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. })),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			// "anything" always contains any subset
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			// reclaiming deposits is part of NonTransfer but not in NonDepositClaiming
			(ProxyType::NonDepositClaiming, ProxyType::NonTransfer) => false,
			// everything except NonTransfer and Any is part of NonDepositClaiming
			(ProxyType::NonDepositClaiming, _) => true,
			// Transfers are part of NonDepositClaiming but not in NonTransfer
			(ProxyType::NonTransfer, ProxyType::NonDepositClaiming) => false,
			// everything except NonDepositClaiming and Any is part of NonTransfer
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = constants::proxy::ProxyDepositBase;
	type ProxyDepositFactor = constants::proxy::ProxyDepositFactor;
	type MaxProxies = constants::proxy::MaxProxies;
	type MaxPending = constants::proxy::MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = constants::proxy::AnnouncementDepositBase;
	type AnnouncementDepositFactor = constants::proxy::AnnouncementDepositFactor;
	type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
}

construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = runtime_common::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system = 0,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip = 1,

		Timestamp: pallet_timestamp = 2,
		Indices: pallet_indices::{Pallet, Call, Storage, Event<T>} = 5,
		Balances: pallet_balances = 6,
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage} = 7,

		// Consensus support.
		// The following order MUST NOT be changed: Authorship -> Staking -> Session -> Aura -> AuraExt
		Authorship: pallet_authorship::{Pallet, Call, Storage} = 20,
		ParachainStaking: parachain_staking = 21,
		Session: pallet_session = 22,
		Aura: pallet_aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// Governance stuff
		Democracy: pallet_democracy = 30,
		Council: pallet_collective::<Instance1> = 31,
		TechnicalCommittee: pallet_collective::<Instance2> = 32,
		// placeholder: parachain council election = 33,
		TechnicalMembership: pallet_membership = 34,
		Treasury: pallet_treasury = 35,

		// Utility module.
		Utility: pallet_utility = 40,

		// Vesting. Usable initially, but removed once all vesting is finished.
		Vesting: pallet_vesting = 41,

		// System scheduler.
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 42,

		// Proxy pallet.
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 43,

		// Preimage registrar
		Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 44,

		// KILT Pallets. Start indices 60 to leave room
		KiltLaunch: kilt_launch = 60,
		Ctype: ctype = 61,
		Attestation: attestation = 62,
		Delegation: delegation = 63,
		Did: did = 64,
		// DELETED: CrowdloanContributors = 65,
		Inflation: pallet_inflation = 66,
		DidLookup: pallet_did_lookup = 67,
		Web3Names: pallet_web3_names = 68,

		// Parachains pallets. Start indices at 80 to leave room.
		ParachainSystem: cumulus_pallet_parachain_system = 80,
		ParachainInfo: parachain_info::{Pallet, Storage, Config} = 81,
	}
}

impl did::DeriveDidCallAuthorizationVerificationKeyRelationship for Call {
	fn derive_verification_key_relationship(&self) -> did::DeriveDidCallKeyRelationshipResult {
		/// ensure that all calls have the same VerificationKeyRelationship
		fn single_key_relationship(calls: &[Call]) -> did::DeriveDidCallKeyRelationshipResult {
			let init = calls
				.get(0)
				.ok_or(did::RelationshipDeriveError::InvalidCallParameter)?
				.derive_verification_key_relationship()?;
			calls
				.iter()
				.skip(1)
				.map(Call::derive_verification_key_relationship)
				.try_fold(init, |acc, next| {
					if next.is_err() {
						next
					} else if Ok(acc) == next {
						Ok(acc)
					} else {
						Err(did::RelationshipDeriveError::InvalidCallParameter)
					}
				})
		}
		match self {
			Call::Attestation { .. } => Ok(did::DidVerificationKeyRelationship::AssertionMethod),
			Call::Ctype { .. } => Ok(did::DidVerificationKeyRelationship::AssertionMethod),
			Call::Delegation { .. } => Ok(did::DidVerificationKeyRelationship::CapabilityDelegation),
			// DID creation is not allowed through the DID proxy.
			Call::Did(did::Call::create { .. }) => Err(did::RelationshipDeriveError::NotCallableByDid),
			Call::Did { .. } => Ok(did::DidVerificationKeyRelationship::Authentication),
			Call::Web3Names { .. } => Ok(did::DidVerificationKeyRelationship::Authentication),
			Call::DidLookup { .. } => Ok(did::DidVerificationKeyRelationship::Authentication),
			Call::Utility(pallet_utility::Call::batch { calls }) => single_key_relationship(&calls[..]),
			Call::Utility(pallet_utility::Call::batch_all { calls }) => single_key_relationship(&calls[..]),
			#[cfg(not(feature = "runtime-benchmarks"))]
			_ => Err(did::RelationshipDeriveError::NotCallableByDid),
			// By default, returns the authentication key
			#[cfg(feature = "runtime-benchmarks")]
			_ => Ok(did::DidVerificationKeyRelationship::Authentication),
		}
	}

	// Always return a System::remark() extrinsic call
	#[cfg(feature = "runtime-benchmarks")]
	fn get_call_for_did_call_benchmark() -> Self {
		Call::System(frame_system::Call::remark { remark: vec![] })
	}
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	// Executes pallet hooks in reverse order of definition in construct_runtime
	// If we want to switch to AllPalletsWithSystem, we need to reorder the staking pallets
	AllPalletsReversedWithSystemFirst,
	pallet_did_lookup::migrations::LookupReverseIndexMigration<Runtime>,
>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			frame_system::Pallet::<Runtime>::account_nonce(&account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(
			extrinsic: <Block as BlockT>::Extrinsic,
		) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}

		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuthorityId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuthorityId> {
			Aura::authorities().into_inner()
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();

			// Substrate
			list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_session, SessionBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_collective, Council);
			list_benchmark!(list, extra, pallet_democracy, Democracy);
			list_benchmark!(list, extra, pallet_indices, Indices);
			list_benchmark!(list, extra, pallet_membership, TechnicalMembership);
			list_benchmark!(list, extra, pallet_preimage, Preimage);
			list_benchmark!(list, extra, pallet_scheduler, Scheduler);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pallet_treasury, Treasury);
			list_benchmark!(list, extra, pallet_utility, Utility);
			list_benchmark!(list, extra, pallet_vesting, Vesting);
			list_benchmark!(list, extra, pallet_proxy, Proxy);

			// KILT
			list_benchmark!(list, extra, attestation, Attestation);
			list_benchmark!(list, extra, ctype, Ctype);
			list_benchmark!(list, extra, delegation, Delegation);
			list_benchmark!(list, extra, did, Did);
			list_benchmark!(list, extra, pallet_did_lookup, DidLookup);
			list_benchmark!(list, extra, kilt_launch, KiltLaunch);
			list_benchmark!(list, extra, pallet_inflation, Inflation);
			list_benchmark!(list, extra, parachain_staking, ParachainStaking);
			list_benchmark!(list, extra, pallet_web3_names, Web3Names);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl cumulus_pallet_session_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
					.to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
					.to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
					.to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
					.to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
					.to_vec().into(),
				// KiltLaunch transfer account
				hex_literal::hex!("6a3c793cec9dbe330b349dc4eea6801090f5e71f53b1b41ad11afb4a313a282c").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			// Substrate
			add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_collective, Council);
			add_benchmark!(params, batches, pallet_democracy, Democracy);
			add_benchmark!(params, batches, pallet_indices, Indices);
			add_benchmark!(params, batches, pallet_membership, TechnicalMembership);
			add_benchmark!(params, batches, pallet_preimage, Preimage);
			add_benchmark!(params, batches, pallet_scheduler, Scheduler);
			add_benchmark!(params, batches, pallet_session, SessionBench::<Runtime>);
			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_treasury, Treasury);
			add_benchmark!(params, batches, pallet_utility, Utility);
			add_benchmark!(params, batches, pallet_vesting, Vesting);
			add_benchmark!(params, batches, pallet_proxy, Proxy);

			// KILT
			add_benchmark!(params, batches, attestation, Attestation);
			add_benchmark!(params, batches, ctype, Ctype);
			add_benchmark!(params, batches, delegation, Delegation);
			add_benchmark!(params, batches, did, Did);
			add_benchmark!(params, batches, pallet_did_lookup, DidLookup);
			add_benchmark!(params, batches, kilt_launch, KiltLaunch);
			add_benchmark!(params, batches, pallet_inflation, Inflation);
			add_benchmark!(params, batches, parachain_staking, ParachainStaking);
			add_benchmark!(params, batches, pallet_web3_names, Web3Names);

			// No benchmarks for these pallets
			// add_benchmark!(params, batches, cumulus_pallet_parachain_system, ParachainSystem);
			// add_benchmark!(params, batches, parachain_info, ParachainInfo);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			log::info!("try-runtime::on_runtime_upgrade spiritnet runtime.");
			let weight = Executive::try_runtime_upgrade().map_err(|err|{
				log::info!("try-runtime::on_runtime_upgrade failed with: {:?}", err);
				err
			}).unwrap();
			(weight, BlockWeights::get().max_block)
		}
		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data = cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
			relay_chain_slot,
			sp_std::time::Duration::from_secs(6),
		)
		.create_inherent_data()
		.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
