use oxydian::prelude::*;

pub const REFRESH: Flow = Flow { 
	name: "refresh",
	func: FreeFn(|vault| {
		let note = Item::get(vault.path("Notes/CX.md"))?.note().ok_or(anyhow!("<!> Not a note"))?.get_content();
		print!("{note}");
		Ok(())
	})
};

pub const REFRESH_ALL: Flow = Flow {
	name: "refresh_all",
	func: FreeFn(|vault| {
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
	})
};
