use oxydian::prelude::*;

fn main() {
	let vault = Vault::new(VaultSetup { root_path:  "~/Obsidian/TestVault".into() });
	if let Err(e) = vault { eprintln!("<!> Error: {}", e); }
}
