use godot::prelude::*;
use godot::classes::Node;

#[derive(GodotClass)]
#[class(base=Node, init)]
struct Player {
    base: Base<Node>
}

#[godot_api]
impl INode for Player
{
    fn ready(&mut self)
    {
        godot_print!("Hello World");
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}