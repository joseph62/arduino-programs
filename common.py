import os
import json
import subprocess
from collections import namedtuple

Config = namedtuple("Config", ["rust_toolchain"])

PROJECT_ROOT = "sketches"
TEMPLATE_ROOT = "rust_template"


def override_rust_toolchain(rust_toolchain, project_directory):
    subprocess.run(
        ["rustup", "override", "set", rust_toolchain], cwd=project_directory, check=True
    )


def get_project_directory(name):
    return f"{PROJECT_ROOT}/{name}"


def is_rust_uno_project(project_directory):
    return os.path.exists(f"{project_directory}/Cargo.toml")


def get_config():
    with open(f"{TEMPLATE_ROOT}/rust-config.json") as f:
        config = json.load(f)
    return Config(config["override-toolchain"])
