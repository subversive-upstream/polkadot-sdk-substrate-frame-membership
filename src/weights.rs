// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=./frame/membership/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_membership.
pub trait WeightInfo {
	fn add_member(m: u32, ) -> Weight;
	fn remove_member(m: u32, ) -> Weight;
	fn swap_member(m: u32, ) -> Weight;
	fn reset_member(m: u32, ) -> Weight;
	fn change_key(m: u32, ) -> Weight;
	fn set_prime(m: u32, ) -> Weight;
	fn clear_prime(m: u32, ) -> Weight;
}

/// Weights for pallet_membership using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		Weight::from_ref_time(15_318_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(51_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_005_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(45_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_029_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(55_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_105_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(158_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		Weight::from_ref_time(18_852_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(55_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(4_869_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(28_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(1_593_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		Weight::from_ref_time(15_318_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(51_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_005_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(45_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_029_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(55_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		Weight::from_ref_time(18_105_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(158_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		Weight::from_ref_time(18_852_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(55_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(4_869_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(28_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(1_593_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).scalar_saturating_mul(m as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
}
