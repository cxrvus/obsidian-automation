use oxydian::prelude::*;

fn main() {
	let app = App::new();
	if let Err(e) = app { eprintln!("<!> Error: {}", e); }
}
