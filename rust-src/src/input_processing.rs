use std::collections::HashMap;
use godot::classes::{IObject, Input, InputMap, Object};
use godot::global::Key;
use godot::obj::{Base, Gd};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct InputWrapper {
    actions_keys: HashMap<String, Key>,
    input: Gd<Input>,
    base: Base<Object>
}

#[godot_api]
impl IObject for InputWrapper {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            actions_keys: HashMap::new(),
            input: Input::singleton(),
            base
        }
    }
}

impl InputWrapper {
    fn setup_actions(&mut self) {
        let mut input_map = InputMap::singleton();
        let actions = input_map.get_actions();

        for action in actions {
            let event = input_map.action_get_events(action);

        }
    }

    pub fn process_input(&self) {

    }
}