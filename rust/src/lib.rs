mod singletons;

use godot::prelude::*;
use godot::classes::{Engine};
use godot::classes::class_macros::sys::InitLevel;
use singletons::window_interface::{WindowInterface, WINDOW_INTERFACE_NAME};

struct GameLibrary;

fn initialize_scene_singletons()
{
    let mut engine: Gd<Engine> = Engine::singleton();
    engine.register_singleton(WINDOW_INTERFACE_NAME, &WindowInterface::new_alloc());
}

fn deinitialize_scene_singletons()
{
    let mut engine: Gd<Engine> = Engine::singleton();

    engine.get_singleton(WINDOW_INTERFACE_NAME).map(|singleton: Gd<Object>| {
        engine.unregister_singleton(WINDOW_INTERFACE_NAME);
        singleton.free();
    });
}

#[gdextension]
unsafe impl ExtensionLibrary for GameLibrary 
{
    
    fn on_level_init(level: InitLevel) 
    {
        match level 
        {
            InitLevel::Scene => initialize_scene_singletons(),
            _ => {}
        }
    }


    fn on_level_deinit(level: InitLevel) 
    {
       match level 
        {
            InitLevel::Scene => deinitialize_scene_singletons(),
            _ => {}
        }
    }
}