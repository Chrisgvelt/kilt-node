// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

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

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_democracy.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Proof: Democracy Blacklist (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	// Storage: Democracy DepositOf (r:0 w:1)
	// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn propose() -> Weight {
		Weight::from_ref_time(41_446_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn second() -> Weight {
		Weight::from_ref_time(38_933_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		Weight::from_ref_time(53_918_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		Weight::from_ref_time(54_304_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy Cancellations (r:1 w:1)
	// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(30_714_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Proof: Democracy Blacklist (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		Weight::from_ref_time(90_310_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Proof: Democracy Blacklist (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		Weight::from_ref_time(15_307_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(3_986_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(5_835_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		Weight::from_ref_time(20_217_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	// Storage: Democracy Blacklist (r:1 w:1)
	// Proof: Democracy Blacklist (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		Weight::from_ref_time(27_110_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		Weight::from_ref_time(75_415_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(14_146_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn on_initialize_base(r: u32, ) -> Weight {
		Weight::from_ref_time(22_311_643 as u64)
			// Standard Error: 27_850
			.saturating_add(Weight::from_ref_time(2_648_560 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		Weight::from_ref_time(27_084_293 as u64)
			// Standard Error: 33_345
			.saturating_add(Weight::from_ref_time(2_650_348 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn delegate(r: u32, ) -> Weight {
		Weight::from_ref_time(53_458_061 as u64)
			// Standard Error: 14_710
			.saturating_add(Weight::from_ref_time(3_806_069 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn undelegate(r: u32, ) -> Weight {
		Weight::from_ref_time(30_373_720 as u64)
			// Standard Error: 9_310
			.saturating_add(Weight::from_ref_time(3_821_210 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(3_948_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn unlock_remove(_r: u32, ) -> Weight {
		Weight::from_ref_time(32_745_173 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn unlock_set(r: u32, ) -> Weight {
		Weight::from_ref_time(32_932_592 as u64)
			// Standard Error: 4_737
			.saturating_add(Weight::from_ref_time(37_294 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	fn remove_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(23_896_077 as u64)
			// Standard Error: 3_788
			.saturating_add(Weight::from_ref_time(28_255 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Proof: Democracy VotingOf (max_values: None, max_size: Some(3799), added: 6274, mode: MaxEncodedLen)
	fn remove_other_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(23_342_468 as u64)
			// Standard Error: 3_631
			.saturating_add(Weight::from_ref_time(44_793 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
