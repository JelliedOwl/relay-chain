// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot CLI library.

use clap::Parser;

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	#[allow(missing_docs)]
	#[clap(name = "prepare-worker", hide = true)]
	PvfPrepareWorker(ValidationWorkerCommand),

	#[allow(missing_docs)]
	#[clap(name = "execute-worker", hide = true)]
	PvfExecuteWorker(ValidationWorkerCommand),

	/// The custom benchmark subcommand benchmarking runtime pallets.
	#[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Benchmark the execution time of historic blocks and compare it to their consumed weight.
	#[clap(
		name = "benchmark-block",
		about = "Benchmark the execution time of historic blocks and compare it to their consumed weight."
	)]
	BenchmarkBlock(frame_benchmarking_cli::BlockCmd),

	/// Sub command for benchmarking the per-block and per-extrinsic execution overhead.
	#[clap(
		name = "benchmark-overhead",
		about = "Benchmark the per-block and per-extrinsic execution overhead."
	)]
	BenchmarkOverhead(frame_benchmarking_cli::OverheadCmd),

	/// Sub command for benchmarking the storage speed.
	#[clap(name = "benchmark-storage", about = "Benchmark storage speed.")]
	BenchmarkStorage(frame_benchmarking_cli::StorageCmd),

	/// Runs performance checks such as PVF compilation in order to measure machine
	/// capabilities of running a validator.
	HostPerfCheck,

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Key management CLI utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),
}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct ValidationWorkerCommand {
	/// The path to the validation host's socket.
	pub socket_path: String,
}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	/// Force using Kusama native runtime.
	#[clap(long = "force-kusama")]
	pub force_kusama: bool,

	/// Force using Westend native runtime.
	#[clap(long = "force-westend")]
	pub force_westend: bool,

	/// Force using Rococo native runtime.
	#[clap(long = "force-rococo")]
	pub force_rococo: bool,

	/// Setup a GRANDPA scheduled voting pause.
	///
	/// This parameter takes two values, namely a block number and a delay (in
	/// blocks). After the given block number is finalized the GRANDPA voter
	/// will temporarily stop voting for new blocks until the given delay has
	/// elapsed (i.e. until a block at height `pause_block + delay` is imported).
	#[clap(long = "grandpa-pause", number_of_values(2))]
	pub grandpa_pause: Vec<u32>,

	/// Enable the BEEFY gadget (only on Rococo or Wococo for now).
	#[clap(long)]
	pub beefy: bool,

	/// Add the destination address to the jaeger agent.
	///
	/// Must be valid socket address, of format `IP:Port`
	/// commonly `127.0.0.1:6831`.
	#[clap(long)]
	pub jaeger_agent: Option<String>,

	/// Add the destination address to the `pyroscope` agent.
	///
	/// Must be valid socket address, of format `IP:Port`
	/// commonly `127.0.0.1:4040`.
	#[clap(long)]
	pub pyroscope_server: Option<String>,
}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,
	#[clap(flatten)]
	pub run: RunCmd,
}
