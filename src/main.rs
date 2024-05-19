use oxydian::prelude::*;
mod flows;

fn main() {
	if let Err(e) = execute() { eprintln!("<!>\n{}", e); }
}

fn execute() -> Result<()> {
	Vault::new(VaultSetup {
			root_path:  "/home/cxrvus/Obsidian/TestVault".into()
		})?
		.with_flows(HashMap::from([
			("refresh".into(), flows::refresh::REFRESH)
		]))?
		.execute()?
	;

	Ok(())
}
