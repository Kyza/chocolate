use core::str;

use web_sys::HtmlElement;

pub mod component;
mod scope;
pub mod signal;

pub fn render(root: fn() -> HtmlElement, location: &HtmlElement) {
	_ = location.append_child(&root());
}
