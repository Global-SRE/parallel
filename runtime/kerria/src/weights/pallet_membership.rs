
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kerria-dev"), DB CACHE: 1024

// Executed Command:
// target/release/parallel
// benchmark
// pallet
// --chain=kerria-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_membership
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/kerria/src/weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalCommitteeMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(14_284_000 as Weight)
			// Standard Error: 0
			.saturating_add((26_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommitteeMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(17_251_000 as Weight)
			// Standard Error: 0
			.saturating_add((18_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommitteeMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(17_279_000 as Weight)
			// Standard Error: 0
			.saturating_add((27_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommitteeMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(17_074_000 as Weight)
			// Standard Error: 0
			.saturating_add((93_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommitteeMembership Prime (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(17_948_000 as Weight)
			// Standard Error: 0
			.saturating_add((23_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Members (r:1 w:0)
	// Storage: TechnicalCommitteeMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(4_278_000 as Weight)
			// Standard Error: 0
			.saturating_add((13_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalCommitteeMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		(1_083_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}