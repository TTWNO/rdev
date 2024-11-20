use rdev::{grab, Event, EventType, Key};

#[derive(Default)]
pub struct ComboSet {
	combos: Vec<Vec<Key>>,
}

#[derive(Default)]
pub struct State {
	activation_key_pressed: bool,
	pressed: Vec<Key>,
	combos: ComboSet,
}

fn main() {
		let mut state = State::default();
		let mut caps_keys = ComboSet::default();
		caps_keys.combos = vec![
			vec![Key::KeyA],
			vec![Key::KeyP, Key::KeyL],
		];
		state.combos = caps_keys;
    // This will block.
    if let Err(error) = grab(callback, state) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event, mut state: &mut State) -> Option<Event> {
		println!("My callback {:?}", event);
		match (event.event_type, state.activation_key_pressed) {
				(EventType::KeyPress(Key::CapsLock), false) => {
						state.activation_key_pressed = true;
						println!("Cancelling CL!");
						None
				},
				(EventType::KeyPress(Key::CapsLock), true) => {
						None
				},
				(EventType::KeyRelease(Key::CapsLock), true) => {
						state.activation_key_pressed = false;
						println!("Cancelling CL! Dropping activation feature.");
						None
				},
				(EventType::KeyPress(other), true) => {
						let None = state.pressed.iter().position(|key| *key == other) else {
								return None;
						};
						state.pressed.push(other);
						for combo in &state.combos.combos {
							println!("Combo: {combo:?}");
							println!("Pressed {:?}", state.pressed);
							if combo == &state.pressed {
								println!("Combo found!");
							}
						}
						None
				},
				(EventType::KeyRelease(other), true) => {
						if let Some(idx) = state.pressed.iter().position(|key| *key == other) {
							state.pressed.remove(idx);
							None
						} else {
							Some(event)
						}
				},
				_ => Some(event),
		}
}
