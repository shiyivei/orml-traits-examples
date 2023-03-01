// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_dex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/dex/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_dex.
pub trait WeightInfo {
	fn enable_trading_pair() -> Weight;
	fn disable_trading_pair() -> Weight;
	fn list_provisioning() -> Weight;
	fn update_provisioning_parameters() -> Weight;
	fn end_provisioning() -> Weight;
	fn add_provision() -> Weight;
	fn claim_dex_share() -> Weight;
	fn add_liquidity() -> Weight;
	fn add_liquidity_and_stake() -> Weight;
	fn remove_liquidity() -> Weight;
	fn remove_liquidity_by_unstake() -> Weight;
	fn swap_with_exact_supply(u: u32, ) -> Weight;
	fn swap_with_exact_target(u: u32, ) -> Weight;
	fn refund_provision() -> Weight;
	fn abort_provisioning() -> Weight;
}

/// Weights for module_dex using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn enable_trading_pair() -> Weight {
		Weight::from_ref_time(24_728_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn disable_trading_pair() -> Weight {
		Weight::from_ref_time(24_891_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn list_provisioning() -> Weight {
		Weight::from_ref_time(37_619_000)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn update_provisioning_parameters() -> Weight {
		Weight::from_ref_time(11_808_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn end_provisioning() -> Weight {
		Weight::from_ref_time(78_617_000)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn add_provision() -> Weight {
		Weight::from_ref_time(127_543_000)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn claim_dex_share() -> Weight {
		Weight::from_ref_time(105_716_000)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn add_liquidity() -> Weight {
		Weight::from_ref_time(184_975_000)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	fn add_liquidity_and_stake() -> Weight {
		Weight::from_ref_time(258_276_000)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	fn remove_liquidity() -> Weight {
		Weight::from_ref_time(158_440_000)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn remove_liquidity_by_unstake() -> Weight {
		Weight::from_ref_time(277_297_000)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		Weight::from_ref_time(93_799_000)
			// Standard Error: 117_000
			.saturating_add(Weight::from_ref_time(16_008_000).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn swap_with_exact_target(u: u32, ) -> Weight {
		Weight::from_ref_time(93_966_000)
			// Standard Error: 226_000
			.saturating_add(Weight::from_ref_time(16_058_000).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn refund_provision() -> Weight {
		Weight::from_ref_time(105_716_000)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn abort_provisioning() -> Weight {
		Weight::from_ref_time(78_617_000)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn enable_trading_pair() -> Weight {
		Weight::from_ref_time(24_728_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn disable_trading_pair() -> Weight {
		Weight::from_ref_time(24_891_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn list_provisioning() -> Weight {
		Weight::from_ref_time(37_619_000)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn update_provisioning_parameters() -> Weight {
		Weight::from_ref_time(11_808_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn end_provisioning() -> Weight {
		Weight::from_ref_time(78_617_000)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn add_provision() -> Weight {
		Weight::from_ref_time(127_543_000)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn claim_dex_share() -> Weight {
		Weight::from_ref_time(105_716_000)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn add_liquidity() -> Weight {
		Weight::from_ref_time(184_975_000)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn add_liquidity_and_stake() -> Weight {
		Weight::from_ref_time(258_276_000)
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	fn remove_liquidity() -> Weight {
		Weight::from_ref_time(158_440_000)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn remove_liquidity_by_unstake() -> Weight {
		Weight::from_ref_time(277_297_000)
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		Weight::from_ref_time(93_799_000)
			// Standard Error: 117_000
			.saturating_add(Weight::from_ref_time(16_008_000).saturating_mul(u as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn swap_with_exact_target(u: u32, ) -> Weight {
		Weight::from_ref_time(93_966_000)
			// Standard Error: 226_000
			.saturating_add(Weight::from_ref_time(16_058_000).saturating_mul(u as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn refund_provision() -> Weight {
		Weight::from_ref_time(105_716_000)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn abort_provisioning() -> Weight {
		Weight::from_ref_time(78_617_000)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
}
