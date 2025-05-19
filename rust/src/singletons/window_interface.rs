use godot::prelude::*;
use godot::classes::{Object, DisplayServer};


#[derive(GodotClass)]
#[class(base = Object, init)]
struct WindowInterface
{
    base: Base<Object>
}

#[godot_api]
impl WindowInterface 
{
    pub(crate) fn window_setup(&mut self)
    {
        todo!()
    }

    pub(crate) fn move_window(&mut self, move_y: f32)
    {
        todo!()
    } 
}