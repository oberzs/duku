import time
import os
import sys
import json
from watchdog.observers import Observer
from watchdog.events import PatternMatchingEventHandler

event_set = set([])
asset_dir = "./assets"
shader_dir = f"{asset_dir}/shaders"
font_dir = f"{asset_dir}/fonts"
font_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.!()[]"


def parse_msdf_output(output):
    result = {}
    lines = output.splitlines()

    # bounds
    bounds = list(map(lambda b: b.strip(), lines[0].split("=")[1].split(",")))
    result["bound_x"] = round(float(bounds[0]))
    result["bound_y"] = round(float(bounds[1]))
    result["bound_w"] = round(float(bounds[2]))
    result["bound_h"] = round(float(bounds[3]))

    # advance
    advance = lines[1].split("=")[1].strip()
    result["advance"] = round(float(advance))

    # range
    range_num = lines[2].split("=")[1].strip()
    result["range"] = int(range_num)

    return result


def compile_shader(path):
    print(f"Compiling {path}")
    out = f"{shader_dir}/spv/{os.path.basename(path)}.spv"
    os.system(f"glslc {path} -o {out} -std=450 --target-env=vulkan1.1")


def compile_font(path):
    print(f"Compiling {path}")
    props = dict(
        x_offset=3,
        y_offset=10,
        chars=[]
    )
    for char in font_chars:
        big = "-big" if char.isupper() else ""
        out = f"{font_dir}/msdf/chars/{char}{big}.png"
        output = os.popen(
            f"msdfgen -font {path} '{char}' -o {out} -size 40 40 -translate {props['x_offset']} {props['y_offset']} -printmetrics").read()
        parsed = parse_msdf_output(output)
        parsed["char"] = char
        props["chars"].append(parsed)
    with open(f"{font_dir}/msdf/props.json", "w") as file:
        file.write(json.dumps(props, indent=4))


def on_shader_modified(event):
    seconds = int(time.time() % 60)
    path = event.src_path
    if (seconds, path) in event_set:
        return
    event_set.add((seconds, path))
    compile_shader(path)


def compile_shaders():
    # Precompile shaders
    frag_dir = f"{shader_dir}/glsl/frag"
    for file in os.listdir(frag_dir):
        path = f"{frag_dir}/{file}"
        compile_shader(path)
    vert_dir = f"{shader_dir}/glsl/vert"
    for file in os.listdir(vert_dir):
        path = f"{vert_dir}/{file}"
        compile_shader(path)

    # Watch assets
    patterns = ["*.frag", "*.vert"]
    ignore_patterns = ["*/objects.glsl"]
    handler = PatternMatchingEventHandler(
        patterns, ignore_patterns, ignore_directories=True, case_sensitive=True)
    handler.on_modified = on_shader_modified

    observer = Observer()
    observer.schedule(handler, asset_dir, recursive=True)
    observer.start()
    try:
        while True:
            time.sleep(1)
            event_set.clear()
    except KeyboardInterrupt:
        observer.stop()
        observer.join()


def compile_fonts():
    compile_font(f"{font_dir}/ttf/NotoSans-Bold.ttf")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        exit(0)
    if sys.argv[1] == "fonts":
        compile_fonts()
    if sys.argv[1] == "shaders":
        compile_shaders()
