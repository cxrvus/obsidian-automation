use oxydian;


fn main() {
	let app = oxydian::flow::App::new();
	if let Err(e) = app { eprintln!("<!> Error: {}", e); }
}
