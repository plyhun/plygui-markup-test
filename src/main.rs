extern crate plygui;
extern crate plygui_markup;

use plygui_markup::*;

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
				"label": "Click It"
			},
			{
				"id": "btn_dont_click_it",
				"type": "Button",
				"label": "Don't Click It"
			}
		]
	}"#;

fn main() {
	use plygui::{UiApplication};
	
	let mut application = plygui::Application::with_name("Plygui markup test");
	
	let mut window = application.new_window("plygui!!", plygui::WindowStartSize::Exact(640, 480), false);
	plygui::register_markup_members(plygui_markup::registry_mut());
	let res = parse_markup(TEST);
	
	window.set_child(Some(res));
	application.start();
}