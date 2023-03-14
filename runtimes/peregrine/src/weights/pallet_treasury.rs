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

//! Autogenerated weights for pallet_treasury
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
// --pallet=pallet-treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_treasury.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	fn spend() -> Weight {
		Weight::from_ref_time(263_000 as u64)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:0 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		Weight::from_ref_time(41_014_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		Weight::from_ref_time(42_921_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn approve_proposal(_p: u32, ) -> Weight {
		Weight::from_ref_time(16_222_330 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		Weight::from_ref_time(8_298_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:201 w:201)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	// Storage: Treasury Deactivated (r:1 w:1)
	// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:100 w:100)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn on_initialize_proposals(p: u32, ) -> Weight {
		Weight::from_ref_time(80_910_880 as u64)
			// Standard Error: 81_695
			.saturating_add(Weight::from_ref_time(34_076_902 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
	}
}
