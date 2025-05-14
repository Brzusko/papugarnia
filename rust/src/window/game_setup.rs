use godot::classes::display_server::WindowFlags;
use godot::prelude::*;
use godot::classes::{DisplayServer, INode, Node, Os};

#[derive(GodotClass)]
#[class(base = Node)]
struct GameSetup
{
    base: Base<Node>,
    os: Gd<Os>,
    display_server: Gd<DisplayServer>,
}

#[godot_api]
impl INode for GameSetup 
{
    fn init(base: Base<Node>) -> Self
    {
        let os: Gd<Os> = Os::singleton();
        let display_server: Gd<DisplayServer> = DisplayServer::singleton();

        Self 
        {
            base,
            os,
            display_server,
        }
    }

    fn ready(&mut self)
    {
        self.setup_cpu();
        self.setup_window();
    }
}

#[godot_api]
impl GameSetup
{
    fn setup_window(&mut self)
    {

        // TODO -- vieport resizing alongisde with viewport control node

        let size: Vector2i = self.display_server.screen_get_size();
        self.display_server.window_set_size(Vector2i::new(size.x, 150));

        self.display_server.window_set_flag(WindowFlags::ALWAYS_ON_TOP, true);
        self.display_server.window_set_flag(WindowFlags::BORDERLESS, true);
        self.display_server.window_set_flag(WindowFlags::RESIZE_DISABLED, true);
        self.display_server.window_set_flag(WindowFlags::TRANSPARENT, true);
        self.display_server.window_set_position(Vector2i { x: 0, y: size.y - 150 });

    }

    fn setup_cpu(&mut self)
    {
        self.os.set_low_processor_usage_mode(true);
    }
}