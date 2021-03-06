
//! Autogenerated weights for `pallet_farming`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("vanilla-dev"), DB CACHE: 1024

// Executed Command:
// ./parallel
// benchmark
// pallet
// --chain=vanilla-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_farming
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/vanilla/src/weights/pallet_farming.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_farming`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_farming::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	fn create() -> Weight {
		(47_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	fn set_pool_status() -> Weight {
		(48_448_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	fn set_pool_cool_down_duration() -> Weight {
		(48_710_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	fn reset_pool_unlock_height() -> Weight {
		(50_720_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(151_532_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	fn withdraw() -> Weight {
		(104_282_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:0)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn redeem() -> Weight {
		(107_091_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim() -> Weight {
		(134_912_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Farming Pools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn dispatch_reward() -> Weight {
		(126_416_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
