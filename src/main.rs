use oxydian::prelude::*;
mod flows;

fn main() {
	if let Err(e) = execute() { eprintln!("<!>\n{}", e); }
}

fn execute() -> Result<()> {
	App::new(VaultConfig {
			root_path:  "/home/cxrvus/Obsidian/TestVault".into()
		})?
		.with_flows(vec![
			flows::REFRESH,
			flows::REFRESH_ALL,
		])?
		.execute()?
	;

	Ok(())
}
