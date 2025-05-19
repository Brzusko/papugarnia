use godot::prelude::*;
use godot::classes::Object;


pub(crate) static WINDOW_INTERFACE_NAME: &str = "WindowInterface";

#[derive(GodotClass)]
#[class(base = Object, init)]
pub(crate) struct WindowInterface
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