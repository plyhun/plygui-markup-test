extern crate plygui;
extern crate plygui_markup;

use plygui_markup::*;

const TEST: &str = r#"
	{
		"id": "window_content",
		"type": "LinearLayout",
		"width": "MatchParent",
		"height": "WrapContent",
		"children": [{
				"id": "btn_click_it",
				"type": "Button",
				"name": "Click It"
			},
			{
				"id": "btn_dont_click_it",
				"type": "Button",
				"name": "Don't Click It"
			}
		]
	}"#;

fn main() {
	let res = parse_markup(TEST, |control_type, content| {
		match control_type {
			"LinearLayout" => {
				plygui::LinearLayout::new(plygui::layout::Orientation::Vertical)
			},
			"Button" => {
				plygui::Button::new("uu")
			},
			_ => panic!("{} type is unsupported", control_type),
		}
	});
}