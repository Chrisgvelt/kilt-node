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

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-14, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=parachain-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/parachain-staking/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	fn on_initialize_no_action() -> Weight;
	fn on_initialize_round_update() -> Weight;
	fn on_initialize_new_year() -> Weight;
	fn on_initialize_network_rewards() -> Weight;
	fn force_new_round() -> Weight;
	fn set_inflation() -> Weight;
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight;
	fn set_blocks_per_round() -> Weight;
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight;
	fn join_candidates(n: u32, m: u32, ) -> Weight;
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight;
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight;
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight;
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight;
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight;
	fn join_delegators(n: u32, m: u32, ) -> Weight;
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight;
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight;
	fn revoke_delegation(n: u32, m: u32, ) -> Weight;
	fn leave_delegators(n: u32, m: u32, ) -> Weight;
	fn unlock_unstaked(u: u32, ) -> Weight;
	fn set_max_candidate_stake() -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ParachainStaking Round (r:1 w:0)
	fn on_initialize_no_action() -> Weight {
		(3_525_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		(14_459_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn on_initialize_new_year() -> Weight {
		(26_228_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		(55_319_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		(1_768_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:0 w:1)
	fn set_inflation() -> Weight {
		(12_952_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((10_570_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 33_000
			.saturating_add((9_858_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		(15_586_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((3_615_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 32_000
			.saturating_add((23_037_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((2_514_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 44_000
			.saturating_add((4_377_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((2_873_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 34_000
			.saturating_add((7_012_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((2_709_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 30_000
			.saturating_add((5_483_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 20_000
			.saturating_add((4_307_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 37_000
			.saturating_add((24_082_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_682_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 41_000
			.saturating_add((7_135_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 246_000
			.saturating_add((2_235_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_502_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 42_000
			.saturating_add((7_182_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking LastDelegation (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 20_000
			.saturating_add((3_803_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 54_000
			.saturating_add((8_007_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 21_000
			.saturating_add((3_810_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 58_000
			.saturating_add((7_940_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 371_000
			.saturating_add((3_810_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((3_567_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 51_000
			.saturating_add((7_292_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn revoke_delegation(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_516_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 45_000
			.saturating_add((7_157_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((3_503_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 49_000
			.saturating_add((7_270_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_unstaked(u: u32, ) -> Weight {
		(33_495_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((366_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		(11_984_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainStaking Round (r:1 w:0)
	fn on_initialize_no_action() -> Weight {
		(3_525_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn on_initialize_round_update() -> Weight {
		(14_459_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn on_initialize_new_year() -> Weight {
		(26_228_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking LastRewardReduction (r:1 w:1)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn on_initialize_network_rewards() -> Weight {
		(55_319_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ParachainStaking ForceNewRound (r:0 w:1)
	fn force_new_round() -> Weight {
		(1_768_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:0 w:1)
	fn set_inflation() -> Weight {
		(12_952_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((10_570_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 33_000
			.saturating_add((9_858_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		(15_586_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((3_615_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 32_000
			.saturating_add((23_037_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(25 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((2_514_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 44_000
			.saturating_add((4_377_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:17 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((2_873_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 34_000
			.saturating_add((7_012_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(21 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((2_709_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 30_000
			.saturating_add((5_483_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking Unstaking (r:36 w:36)
	// Storage: ParachainStaking DelegatorState (r:35 w:35)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session DisabledValidators (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: ParachainStaking CounterForCandidatePool (r:1 w:1)
	fn execute_leave_candidates(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 20_000
			.saturating_add((4_307_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 37_000
			.saturating_add((24_082_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(m as Weight)))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_682_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 41_000
			.saturating_add((7_135_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 246_000
			.saturating_add((2_235_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_502_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 42_000
			.saturating_add((7_182_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:2 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking LastDelegation (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 20_000
			.saturating_add((3_803_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 54_000
			.saturating_add((8_007_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 21_000
			.saturating_add((3_810_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 58_000
			.saturating_add((7_940_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 371_000
			.saturating_add((3_810_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((3_567_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 51_000
			.saturating_add((7_292_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn revoke_delegation(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 17_000
			.saturating_add((3_516_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 45_000
			.saturating_add((7_157_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: ParachainStaking TopCandidates (r:1 w:1)
	// Storage: ParachainStaking MaxSelectedCandidates (r:1 w:0)
	// Storage: ParachainStaking TotalCollatorStake (r:1 w:1)
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((3_503_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 49_000
			.saturating_add((7_270_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking Unstaking (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_unstaked(u: u32, ) -> Weight {
		(33_495_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((366_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainStaking MaxCollatorCandidateStake (r:0 w:1)
	fn set_max_candidate_stake() -> Weight {
		(11_984_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
