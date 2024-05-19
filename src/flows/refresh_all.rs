use oxydian::prelude::*;

pub const REFRESH_ALL: Flow = Flow::Free(|vault| {
	let notes = vault.ls("Notes")?;
	let mut notes = notes.iter()
			.take(10)
		.map(|item| item.name())
		.collect::<Vec<_>>()
	;
	notes.sort();
	let notes = notes.join("\n");
	print!("{}\n", notes);
	Ok(())
});
