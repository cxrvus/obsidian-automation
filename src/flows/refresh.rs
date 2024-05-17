use oxydian::prelude::*;

pub const REFRESH: Flow = Flow::Free(|vault| {
	let note = Item::get(vault.path("Notes/CX.md"))?.note().ok_or(anyhow!("<!> Not a note"))?.get_content();
	print!("{note}");
	Ok(())
});
