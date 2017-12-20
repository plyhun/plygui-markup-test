extern crate plygui;

use plygui::{UiButton, UiApplication};
use plygui::markup::{self, MarkupRegistry, CallbackPtr};

const TEST: &str = r#"
	{
		"id": "window_content",
		"type": "LinearLayout",
		"width": "MatchParent",
		"height": "WrapContent",
		"orientation": "Vertical",
		"children": [{
				"id": "btn_click_it",
				"type": "Button",
				"label": "Click It",
				"on_left_click": "click_da_buttn"
			},
			{
				"id": "btn_dont_click_it",
				"type": "Button",
				"label": "Don't Click It",
				"on_left_click": "click_da_buttn"
			}
		]
	}"#;
	
fn click_da_buttn(button: &mut UiButton) {
	println!("{} bein clickd", button.label());
}

fn main() {
	let mut registry = MarkupRegistry::new();
	plygui::register_markup_members(&mut registry);
	registry.bind_callback("click_da_buttn", click_da_buttn as CallbackPtr).unwrap();
	
	// Closures do not work :(
	//let click_da_other_buttn = Box::new(|button: &mut UiButton| {println!("{} bein super clickd", button.label());});
	//registry.bind_callback("click_da_other_buttn", click_da_other_buttn.as_ref() as &_ as *const _ as CallbackPtr).unwrap();
	
	//https://github.com/rust-lang/rust/issues/29638
	//bind_markup_callback!(registry, click_da_buttn);
	
	let mut application = plygui::Application::with_name("Plygui markup test");
	
	let mut window = application.new_window("plygui!!", plygui::WindowStartSize::Exact(640, 480), false);
	let res = markup::parse_markup(TEST, &mut registry);
	
	window.set_child(Some(res));
	application.start();
}