use godot::prelude::*;
use godot::classes::{INode, Node, Engine};

use crate::singletons::window_interface::{WindowInterface, WINDOW_INTERFACE_NAME};

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct GameSetup
{
    base: Base<Node>,
}

#[godot_api]
impl INode for GameSetup
{
    fn ready(&mut self)
    {
        let engine: Gd<Engine> = Engine::singleton();

        engine.get_singleton(WINDOW_INTERFACE_NAME).map(|window: Gd<Object>| {
            let mut window_interface: Gd<WindowInterface> = window.cast::<WindowInterface>();
            window_interface.bind_mut().window_setup();
        });
    }
}
