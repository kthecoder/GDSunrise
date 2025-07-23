import os
import platform
from SCons.Script import ARGUMENTS, Environment

libname = "web_pink"
demo_folder = "demo"

build_type = ARGUMENTS.get("build", "debug")
env = Environment(ENV=os.environ)

# Auto-detect platform
platform_name = ARGUMENTS.get("platform", platform.system().lower())  # windows/linux/darwin
target_dir = f"target/{build_type}"
output_dir = f"{demo_folder}/bin/{platform_name}/"

# Ensure output directory exists
if platform_name == "windows":
    env.Execute(f"if not exist {output_dir} mkdir {output_dir}")  # Windows-specific
else:
    env.Execute(f"mkdir -p {output_dir}")  # Linux/macOS

# Build Rust GDExtension
env.Command("build", [], f"cargo build {'--release' if build_type == 'release' else ''}")

# Copy compiled binaries using SCons
if platform_name == "windows":
    # Match any file that starts with the library name. This should capture:
    # web_pink.dll, web_pink.dll.exp, web_pink.dll.lib, web_pink.pdb
    library = env.Glob(f"{target_dir}/{libname}*")
else:
    # On Linux/macOS, pick up the .so or .dylib files
    library = env.Glob(f"{target_dir}/*.so") + env.Glob(f"{target_dir}/*.dylib")

env.Install(output_dir, library)

# Set executable permissions on Linux/macOS if needed
if platform_name in ["linux", "darwin"]:
    env.Execute(f"chmod +x {output_dir}/*.so {output_dir}/*.dylib")