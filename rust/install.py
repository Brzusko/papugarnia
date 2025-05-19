import sys;
import shutil;
import os;

# sys.argv[0] - install.py
# sys.argv[1] - build target
# sys.argv[2] - is release build
# if present sys.argv[3] - is cross compile build

TARGET = sys.argv[1]
IS_RELEASE = sys.argv[2]
CAN_CROSS_COMPILE = False

bool_parse_keys = ["true", "True", "y", "Y"]

supported_targets = [
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "aarch64-apple-darwin",
]

file_extensions_per_target = {
    supported_targets[0] : "dll",
    supported_targets[1] : "so",
    supported_targets[2] : "dylib",
}

print(sys.argv)

if len(sys.argv) >= 4:
    CAN_CROSS_COMPILE = sys.argv[3] in bool_parse_keys

if not isinstance(IS_RELEASE, bool):
    IS_RELEASE = IS_RELEASE in bool_parse_keys

def install():
    if TARGET not in supported_targets:
        print("Provided target is not supported")
        return
    
    target_sub_path = "release" if IS_RELEASE else "debug"

    file_name = f"rust.{file_extensions_per_target[TARGET]}"

    source_path = f"./target/{TARGET}/{target_sub_path}/{file_name}"
    destination_path = "../godot/libs/rust/release/" if IS_RELEASE else "../rust-lib/debug/"

    full_path = os.path.join(destination_path, file_name);

    if os.path.exists(full_path):
        os.remove(full_path);

    
    shutil.move(source_path, destination_path)

# TODO
def install_cross():
    pass


if CAN_CROSS_COMPILE is False:
    install()
else:
    install_cross()




