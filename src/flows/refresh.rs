use oxydian::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ViewNote {
	pub view: String,
	pub refreshed: Option<String>,
	pub error: Option<String>,
}

pub const REFRESH: Flow = Flow { 
	name: "refresh",
	func: MutatingFn(|_vault, note| {
		let note = note.note()?;
		let props = note.get_props::<ViewNote>();
		let props = props.ok_or(anyhow!("Provided note is not a View Note"))?;
		print!("{:?}\n\n\n{}", props, note.get_content());
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
