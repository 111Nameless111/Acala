// This file is part of Acala.

// Copyright (C) 2020-2025 Acala Foundation.
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

//! Autogenerated weights for module_evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-38-126`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_evm.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_evm::WeightInfo for WeightInfo<T> {
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3591`
		//  Estimated: `9531`
		// Minimum execution time: 138_774 nanoseconds.
		Weight::from_parts(141_601_000, 9531)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create2() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3591`
		//  Estimated: `9531`
		// Minimum execution time: 133_311 nanoseconds.
		Weight::from_parts(137_004_000, 9531)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::NetworkContractIndex` (r:1 w:1)
	// Proof: `EVM::NetworkContractIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_nft_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3759`
		//  Estimated: `9699`
		// Minimum execution time: 164_646 nanoseconds.
		Weight::from_parts(167_335_000, 9699)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: `EVM::Accounts` (r:2 w:2)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::AccountStorages` (r:1 w:0)
	// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_predeploy_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3930`
		//  Estimated: `9870`
		// Minimum execution time: 163_932 nanoseconds.
		Weight::from_parts(166_386_000, 9870)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:2 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Digest` (r:1 w:0)
	// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:1 w:0)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4284`
		//  Estimated: `10224`
		// Minimum execution time: 131_121 nanoseconds.
		Weight::from_parts(134_446_000, 10224)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn transfer_maintainer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2446`
		//  Estimated: `5911`
		// Minimum execution time: 88_573 nanoseconds.
		Weight::from_parts(89_939_000, 5911)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn publish_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3658`
		//  Estimated: `7123`
		// Minimum execution time: 126_629 nanoseconds.
		Weight::from_parts(128_567_000, 7123)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn publish_free() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2304`
		//  Estimated: `5769`
		// Minimum execution time: 26_287 nanoseconds.
		Weight::from_parts(27_068_000, 5769)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn enable_contract_development() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2229`
		//  Estimated: `3633`
		// Minimum execution time: 93_818 nanoseconds.
		Weight::from_parts(94_547_000, 3633)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn disable_contract_development() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2301`
		//  Estimated: `3633`
		// Minimum execution time: 95_481 nanoseconds.
		Weight::from_parts(96_533_000, 3633)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:2 w:2)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:2 w:2)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:2)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4202`
		//  Estimated: `10157`
		// Minimum execution time: 155_320 nanoseconds.
		Weight::from_parts(151_963_249, 10157)
			// Standard Error: 12
			.saturating_add(Weight::from_parts(5_472, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EvmAccounts::EvmAddresses` (r:2 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::Accounts` (r:1 w:1)
	// Proof: `EVM::Accounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::Accounts` (r:2 w:0)
	// Proof: `EvmAccounts::Accounts` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `EVM::CodeInfos` (r:1 w:1)
	// Proof: `EVM::CodeInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::ContractStorageSizes` (r:1 w:1)
	// Proof: `EVM::ContractStorageSizes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `IdleScheduler::NextTaskId` (r:1 w:1)
	// Proof: `IdleScheduler::NextTaskId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:0)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `IdleScheduler::Tasks` (r:0 w:1)
	// Proof: `IdleScheduler::Tasks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EVM::Codes` (r:0 w:1)
	// Proof: `EVM::Codes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn selfdestruct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4776`
		//  Estimated: `8241`
		// Minimum execution time: 177_970 nanoseconds.
		Weight::from_parts(180_653_000, 8241)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
