use godot::classes::display_server::WindowFlags;
use godot::prelude::*;
use godot::classes::{Object, DisplayServer};


pub(crate) static WINDOW_INTERFACE_NAME: &str = "WindowInterface";

struct WindowSaveData
{
    x_screen_cord: i32,
    y_screen_cord: i32,
    screen_id: i32,
}

impl WindowSaveData
{
    pub fn new(window_interface: &Gd<WindowInterface>) -> Self
    {
        todo!()
    }
}

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
        // TODO -- load from file, save to file uppon any action with the screen
        // data to save/load
        // x,y of screen
        // screen id, if missing assign to main and override
        // TODO END
        let window_height: i32 = 150;
        let mut display_server: Gd<DisplayServer> = DisplayServer::singleton();
        let screen_size: Vector2i = display_server.screen_get_size();

        display_server.window_set_size(Vector2i { x: screen_size.x, y: window_height });
        display_server.window_set_position(Vector2i { x: 0, y: screen_size.y - window_height });
        
        display_server.window_set_flag(WindowFlags::ALWAYS_ON_TOP, true);
        display_server.window_set_flag(WindowFlags::BORDERLESS, true);
        display_server.window_set_flag(WindowFlags::RESIZE_DISABLED, true);
        display_server.window_set_flag(WindowFlags::TRANSPARENT, true);
        display_server.window_set_flag(WindowFlags::MOUSE_PASSTHROUGH, true);
    }

    pub(crate) fn move_window(&mut self, move_y: f32)
    {
        todo!()
    } 
}