#! /usr/bin/env python3

import sys
import argparse
import signal
import subprocess
import shutil
import os
from common import (
    PROJECT_ROOT,
    TEMPLATE_ROOT,
    get_config,
    override_rust_toolchain,
    get_project_directory,
)


def parse_arguments(args):
    parser = argparse.ArgumentParser(description="Create another rust uno project")
    parser.add_argument("name", help="The name of the new project")
    return parser.parse_args(args)


def create_rust_project(name):
    subprocess.run(("cargo", "new", "--bin", name), check=True, cwd=PROJECT_ROOT)


def update_cargo_toml(project_directory):
    with open(f"{project_directory}/Cargo.toml", "r") as f:
        cargo_lines = [line.strip() for line in f]

    with open(f"{TEMPLATE_ROOT}/Partial_Cargo.toml", "r") as f:
        new_cargo_lines = [line.strip() for line in f]

    cargo_lines.extend(new_cargo_lines)

    with open(f"{project_directory}/Cargo.toml", "w") as f:
        f.write("\n".join(cargo_lines))


def main(args):
    args = parse_arguments(args)
    config = get_config()

    name = args.name
    project_directory = get_project_directory(name)
    rust_toolchain = config.rust_toolchain

    if os.path.exists(project_directory):
        print(f"Project '{name}' already exists")
        return 1

    create_rust_project(name)

    update_cargo_toml(project_directory)

    shutil.copytree(f"{TEMPLATE_ROOT}/.cargo", f"{project_directory}/.cargo")

    shutil.copy(
        f"{TEMPLATE_ROOT}/avr-atmega328p.json",
        f"{project_directory}/avr-atmega328p.json",
    )
    shutil.copy(f"{TEMPLATE_ROOT}/src/main.rs", f"{project_directory}/src/main.rs")

    override_rust_toolchain(rust_toolchain, project_directory)

    return 0


if __name__ == "__main__":
    signal.signal(signal.SIGPIPE, signal.SIG_DFL)
    sys.exit(main(sys.argv[1:]))
