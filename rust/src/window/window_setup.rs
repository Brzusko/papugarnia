use godot::prelude::*;
use godot::classes::{INode, Node, Os};

#[derive(GodotClass)]
#[class(base = Node)]
struct WindowSetup
{
    base: Base<Node>,
    os: Gd<Os>,
}

#[godot_api]
impl INode for WindowSetup 
{
    fn init(base: Base<Node>) -> Self
    {
        let os: Gd<Os> = Os::singleton();

        Self 
        {
            base,
            os,
        }
    }

    fn ready(&mut self)
    {
        self.resize_full_width();
    }
}

#[godot_api]
impl WindowSetup
{
    fn resize_full_width(&mut self)
    {
        todo!()
    }
}