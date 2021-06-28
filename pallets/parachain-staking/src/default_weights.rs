// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-28, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 128

// Executed Command:
// /home/willi/mashnet-node/target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --extrinsic=*
// --pallet=parachain_staking
// --steps=50
// --repeat=20
// --output
// ../../pallets/parachain-staking/src/default_weights.rs
// --template
// ../../.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
	fn on_initialize_no_action() -> Weight;
	fn on_initialize_round_update() -> Weight;
	fn on_initialize_new_year() -> Weight;
	fn set_inflation() -> Weight;
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight;
	fn set_blocks_per_round() -> Weight;
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight;
	fn join_candidates(n: u32, m: u32, ) -> Weight;
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight;
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight;
	fn execute_leave_candidates(n: u32, m: u32, u: u32, ) -> Weight;
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight;
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight;
	fn join_delegators(n: u32, m: u32, ) -> Weight;
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight;
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight;
	fn revoke_delegation(n: u32, m: u32, ) -> Weight;
	fn leave_delegators(n: u32, m: u32, ) -> Weight;
	fn unlock_unstaked(u: u32, ) -> Weight;
	fn increase_max_candidate_stake_by() -> Weight;
	fn decrease_max_candidate_stake_by(n: u32, m: u32, ) -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize_no_action() -> Weight {
		(6_733_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn on_initialize_round_update() -> Weight {
		(29_616_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn on_initialize_new_year() -> Weight {
		(55_755_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn set_inflation() -> Weight {
		(25_778_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 91_000
			.saturating_add((26_609_000_u64).saturating_mul(n as Weight))
			// Standard Error: 255_000
			.saturating_add((17_315_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn set_blocks_per_round() -> Weight {
		(28_473_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		(251_862_000_u64)
			// Standard Error: 91_000
			.saturating_add((3_046_000_u64).saturating_mul(n as Weight))
			// Standard Error: 252_000
			.saturating_add((40_903_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m as Weight)))
	}
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		(203_754_000_u64)
			// Standard Error: 111_000
			.saturating_add((4_533_000_u64).saturating_mul(n as Weight))
			// Standard Error: 407_000
			.saturating_add((9_959_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		(381_531_000_u64)
			// Standard Error: 53_000
			.saturating_add((1_516_000_u64).saturating_mul(n as Weight))
			// Standard Error: 144_000
			.saturating_add((6_447_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(22_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		(400_898_000_u64)
			// Standard Error: 54_000
			.saturating_add((1_327_000_u64).saturating_mul(n as Weight))
			// Standard Error: 146_000
			.saturating_add((5_798_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(21_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn execute_leave_candidates(n: u32, m: u32, _u: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 67_000
			.saturating_add((2_053_000_u64).saturating_mul(n as Weight))
			// Standard Error: 182_000
			.saturating_add((36_236_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m as Weight)))
	}
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(130_046_000_u64)
			// Standard Error: 105_000
			.saturating_add((4_512_000_u64).saturating_mul(n as Weight))
			// Standard Error: 391_000
			.saturating_add((9_939_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_296_000
			.saturating_add((6_648_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		(126_968_000_u64)
			// Standard Error: 110_000
			.saturating_add((4_504_000_u64).saturating_mul(n as Weight))
			// Standard Error: 404_000
			.saturating_add((10_259_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		(202_705_000_u64)
			// Standard Error: 112_000
			.saturating_add((4_614_000_u64).saturating_mul(n as Weight))
			// Standard Error: 464_000
			.saturating_add((11_472_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(136_837_000_u64)
			// Standard Error: 117_000
			.saturating_add((4_930_000_u64).saturating_mul(n as Weight))
			// Standard Error: 491_000
			.saturating_add((11_077_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_680_000
			.saturating_add((3_605_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		(340_471_000_u64)
			// Standard Error: 168_000
			.saturating_add((2_328_000_u64).saturating_mul(n as Weight))
			// Standard Error: 691_000
			.saturating_add((8_725_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn revoke_delegation(n: u32, m: u32, ) -> Weight {
		(160_998_000_u64)
			// Standard Error: 117_000
			.saturating_add((4_430_000_u64).saturating_mul(n as Weight))
			// Standard Error: 482_000
			.saturating_add((10_870_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		(175_623_000_u64)
			// Standard Error: 109_000
			.saturating_add((4_374_000_u64).saturating_mul(n as Weight))
			// Standard Error: 451_000
			.saturating_add((10_166_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn unlock_unstaked(u: u32, ) -> Weight {
		(63_961_000_u64)
			// Standard Error: 119_000
			.saturating_add((33_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn increase_max_candidate_stake_by() -> Weight {
		(26_109_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn decrease_max_candidate_stake_by(n: u32, m: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 140_000
			.saturating_add((65_315_000_u64).saturating_mul(n as Weight))
			// Standard Error: 513_000
			.saturating_add((45_189_000_u64).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize_no_action() -> Weight {
		(6_733_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn on_initialize_round_update() -> Weight {
		(29_616_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn on_initialize_new_year() -> Weight {
		(55_755_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn set_inflation() -> Weight {
		(25_778_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_max_selected_candidates(n: u32, m: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 91_000
			.saturating_add((26_609_000_u64).saturating_mul(n as Weight))
			// Standard Error: 255_000
			.saturating_add((17_315_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn set_blocks_per_round() -> Weight {
		(28_473_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn force_remove_candidate(n: u32, m: u32, ) -> Weight {
		(251_862_000_u64)
			// Standard Error: 91_000
			.saturating_add((3_046_000_u64).saturating_mul(n as Weight))
			// Standard Error: 252_000
			.saturating_add((40_903_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(22_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(m as Weight)))
	}
	fn join_candidates(n: u32, m: u32, ) -> Weight {
		(203_754_000_u64)
			// Standard Error: 111_000
			.saturating_add((4_533_000_u64).saturating_mul(n as Weight))
			// Standard Error: 407_000
			.saturating_add((9_959_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(17_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	fn init_leave_candidates(n: u32, m: u32, ) -> Weight {
		(381_531_000_u64)
			// Standard Error: 53_000
			.saturating_add((1_516_000_u64).saturating_mul(n as Weight))
			// Standard Error: 144_000
			.saturating_add((6_447_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(22_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn cancel_leave_candidates(n: u32, m: u32, ) -> Weight {
		(400_898_000_u64)
			// Standard Error: 54_000
			.saturating_add((1_327_000_u64).saturating_mul(n as Weight))
			// Standard Error: 146_000
			.saturating_add((5_798_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(21_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn execute_leave_candidates(n: u32, m: u32, _u: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 67_000
			.saturating_add((2_053_000_u64).saturating_mul(n as Weight))
			// Standard Error: 182_000
			.saturating_add((36_236_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(m as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(m as Weight)))
	}
	fn candidate_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(130_046_000_u64)
			// Standard Error: 105_000
			.saturating_add((4_512_000_u64).saturating_mul(n as Weight))
			// Standard Error: 391_000
			.saturating_add((9_939_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_296_000
			.saturating_add((6_648_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	fn candidate_stake_less(n: u32, m: u32, ) -> Weight {
		(126_968_000_u64)
			// Standard Error: 110_000
			.saturating_add((4_504_000_u64).saturating_mul(n as Weight))
			// Standard Error: 404_000
			.saturating_add((10_259_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	fn join_delegators(n: u32, m: u32, ) -> Weight {
		(202_705_000_u64)
			// Standard Error: 112_000
			.saturating_add((4_614_000_u64).saturating_mul(n as Weight))
			// Standard Error: 464_000
			.saturating_add((11_472_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(18_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	fn delegator_stake_more(n: u32, m: u32, u: u32, ) -> Weight {
		(136_837_000_u64)
			// Standard Error: 117_000
			.saturating_add((4_930_000_u64).saturating_mul(n as Weight))
			// Standard Error: 491_000
			.saturating_add((11_077_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_680_000
			.saturating_add((3_605_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	fn delegator_stake_less(n: u32, m: u32, ) -> Weight {
		(340_471_000_u64)
			// Standard Error: 168_000
			.saturating_add((2_328_000_u64).saturating_mul(n as Weight))
			// Standard Error: 691_000
			.saturating_add((8_725_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn revoke_delegation(n: u32, m: u32, ) -> Weight {
		(160_998_000_u64)
			// Standard Error: 117_000
			.saturating_add((4_430_000_u64).saturating_mul(n as Weight))
			// Standard Error: 482_000
			.saturating_add((10_870_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn leave_delegators(n: u32, m: u32, ) -> Weight {
		(175_623_000_u64)
			// Standard Error: 109_000
			.saturating_add((4_374_000_u64).saturating_mul(n as Weight))
			// Standard Error: 451_000
			.saturating_add((10_166_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn unlock_unstaked(u: u32, ) -> Weight {
		(63_961_000_u64)
			// Standard Error: 119_000
			.saturating_add((33_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn increase_max_candidate_stake_by() -> Weight {
		(26_109_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn decrease_max_candidate_stake_by(n: u32, m: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 140_000
			.saturating_add((65_315_000_u64).saturating_mul(n as Weight))
			// Standard Error: 513_000
			.saturating_add((45_189_000_u64).saturating_mul(m as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
}
