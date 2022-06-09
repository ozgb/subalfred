// crates.io
use async_std::task;
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::cargo;

/// Update the workspace members' crate versions.
/// Pretty useful while you are going to publish a new release.
#[derive(Debug, Args)]
pub struct UpdateCmd {
	/// Path to the root `Cargo.toml`.
	#[clap(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: String,
	/// Release version to update to.
	#[clap(required = true, value_name = "VERSION")]
	version: String,
}
impl UpdateCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { manifest_path, version } = self;

		task::block_on(cargo::update_members_versions(manifest_path, version))?;

		Ok(())
	}
}