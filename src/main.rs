extern crate plygui_markup;

use plygui_markup::*;
use plygui_markup::markup::{self, MarkupRegistry};
use plygui_markup::callbacks;

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
				"on_click": "click_da_buttn"
			},
			{
				"id": "btn_dont_click_it",
				"type": "Button",
				"label": "Don't Click It",
				"on_click": "click_da_other_buttn"
			},
			{
				"id": "the_frame",
				"width": "MatchParent",
				"height": "WrapContent",
				"type": "Frame",
				"label": "Pam pam pam pam",
				"child": {
					"id": "btn_never_click_it",
					"type": "Button",
					"label": "Never ever Click It",
					"on_click": "click_da_3rd_buttn"
				}
			}
		]
	}"#;

fn click_da_buttn(button: &mut dyn Clickable) {
    println!("{} bein clickd", button.as_any().downcast_ref::<imp::Button>().unwrap().label());
}

fn main() {
    let mut registry = MarkupRegistry::new();
    plygui_markup::register_markup_members(&mut registry);

    registry
        .push_callback("click_da_buttn", callbacks::OnClick::from(click_da_buttn))
        .unwrap();
    registry
        .push_callback(
            "click_da_other_buttn",
            callbacks::OnClick::from(|button: &mut dyn Clickable| {
                println!("{} bein super clickd", button.as_any().downcast_ref::<imp::Button>().unwrap().label());
            }),
        )
        .unwrap();
	registry
        .push_callback("click_da_3rd_buttn", callbacks::OnClick::from(click_da_buttn))
        .unwrap();
    
    //https://github.com/rust-lang/rust/issues/29638
    //bind_markup_callback!(registry, click_da_buttn);

    let mut application = plygui_markup::imp::Application::get().unwrap();

    let mut window = application.new_window("plygui!!", plygui_markup::WindowStartSize::Exact(640, 480), None);
    let res = markup::parse_markup(TEST, &mut registry);

    window.set_child(Some(res));
    application.start();
}
