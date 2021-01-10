#! /usr/bin/env python3

import sys
import argparse
import signal
import subprocess
import shutil
import os


def create_rust_project(name):
    subprocess.run(("cargo", "new", "--bin", name), check=True, cwd="sketches")


def parse_arguments(args):
    parser = argparse.ArgumentParser(description="Create another rust uno project")
    parser.add_argument("-n", "--name", required=True)
    return parser.parse_args(args)


def update_cargo_toml(name):
    with open(f"sketches/{name}/Cargo.toml", "r") as f:
        cargo_lines = [line.strip() for line in f]

    with open(f"rust_template/Partial_Cargo.toml", "r") as f:
        new_cargo_lines = [line.strip() for line in f]

    cargo_lines.extend(new_cargo_lines)

    with open(f"sketches/{name}/Cargo.toml", "w") as f:
        f.write("\n".join(cargo_lines))


def main(args):
    args = parse_arguments(args)

    if os.path.exists(args.name):
        print(f"Project '{name}' already exists")
        return 1

    create_rust_project(args.name)

    update_cargo_toml(args.name)

    shutil.copytree("rust_template/.cargo", f"sketches/{args.name}/.cargo")

    shutil.copy(
        "rust_template/avr-atmega328p.json", f"sketches/{args.name}/avr-atmega328p.json"
    )
    shutil.copy("rust_template/src/main.rs", f"sketches/{args.name}/src/main.rs")

    return 0


if __name__ == "__main__":
    signal.signal(signal.SIGPIPE, signal.SIG_DFL)
    sys.exit(main(sys.argv[1:]))
