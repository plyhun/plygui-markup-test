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
	
	let mut application = plygui::Application::with_name("Plygui markup test");
	
	let mut window = application.new_window("plygui!!", plygui::WindowStartSize::Exact(640, 480), false);
	let res = markup::parse_markup(TEST, &mut registry);
	
	window.set_child(Some(res));
	application.start();
}