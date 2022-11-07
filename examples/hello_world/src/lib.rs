use chocolate::signal::create_signal;
// use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
extern "C" {
	// Use `js_namespace` here to bind `console.log(..)` instead of just
	// `log(..)`
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
	let window = web_sys::window().expect("no global `window` exists");
	let document =
		window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let button = document
		.create_element("button")?
		.dyn_into::<HtmlElement>()?;
	button.set_text_content(Some("Count Up"));
	body.append_child(&button)?;

	let mut sig = create_signal(0);

	let count_p = document.create_element("p")?.dyn_into::<HtmlElement>()?;
	count_p.set_text_content(Some("0"));
	body.append_child(&count_p)?;

	let cp = count_p.clone();
	sig.subscribe(move |old, new| {
		cp.set_text_content(Some(new.to_string().as_str()));
		console_log!("{old} -> {new}");
	});

	// count_p.set_text_content(Some("works"));

	let a = Closure::<dyn FnMut()>::new(move || {
		sig.set(*sig.get() + 1);
	});
	button.set_onclick(Some(a.as_ref().unchecked_ref()));

	sig.get();

	a.forget();

	Ok(())
}
