import time
import os
import sys
from watchdog.observers import Observer
from watchdog.events import PatternMatchingEventHandler

event_set = set([])
asset_dir = "./assets"
shader_dir = f"{asset_dir}/shaders"


def compile_shader(path):
    print(f"Compiling {path}")
    out = f"{shader_dir}/spv/{os.path.basename(path)}.spv"
    os.system(f"glslc {path} -o {out} -std=450 --target-env=vulkan1.1")


def on_shader_modified(event):
    seconds = int(time.time() % 60)
    path = event.src_path
    if (seconds, path) in event_set:
        return
    event_set.add((seconds, path))
    compile_shader(path)


if __name__ == "__main__":
    # Precompile everything
    frag_dir = f"{shader_dir}/glsl/frag"
    for file in os.listdir(frag_dir):
        path = f"{frag_dir}/{file}"
        compile_shader(path)
    vert_dir = f"{shader_dir}/glsl/vert"
    for file in os.listdir(vert_dir):
        path = f"{vert_dir}/{file}"
        compile_shader(path)

    if len(sys.argv) < 2 or sys.argv[1] != "watch":
        exit(1)

    # Watch assets
    patterns = ["*.frag", "*.vert"]
    ignore_patterns = ["*/objects.glsl"]
    ignore_directories = True
    case_sensitive = True

    shader_handler = PatternMatchingEventHandler(
        patterns, ignore_patterns, ignore_directories, case_sensitive)
    shader_handler.on_modified = on_shader_modified

    observer = Observer()
    observer.schedule(shader_handler, asset_dir, recursive=True)

    observer.start()
    try:
        while True:
            time.sleep(1)
            event_set.clear()
    except KeyboardInterrupt:
        observer.stop()
        observer.join()
