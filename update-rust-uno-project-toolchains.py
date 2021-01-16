#! /usr/bin/env python3

import sys
import argparse
import signal
import os
from common import (
    get_config,
    PROJECT_ROOT,
    override_rust_toolchain,
    is_rust_uno_project,
    get_project_directory,
)


def parse_arguments(args):
    parser = argparse.ArgumentParser(
        description="Update all arduino project toolchains"
    )
    return parser.parse_args(args)


def main(args):
    args = parse_arguments(args)
    config = get_config()
    projects = [get_project_directory(path) for path in os.listdir(PROJECT_ROOT)]
    for project in projects:
        if is_rust_uno_project(project):
            override_rust_toolchain(config.rust_toolchain, project)
    return 0


if __name__ == "__main__":
    signal.signal(signal.SIGPIPE, signal.SIG_DFL)
    sys.exit(main(sys.argv[1:]))
