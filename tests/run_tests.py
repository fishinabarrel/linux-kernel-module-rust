#!/usr/bin/env python

import os
import subprocess


BASE_DIR = os.path.realpath(os.path.dirname(__file__))


def run(*args, **kwargs):
    cwd = kwargs.pop("cwd", None)
    environ = kwargs.pop("environ", os.environ)
    assert not kwargs

    subprocess.check_call(list(args), cwd=cwd, env=environ)

def main():
    for path in os.listdir(BASE_DIR):
        if (
            not os.path.isdir(os.path.join(BASE_DIR, path)) or
            not os.path.exists(os.path.join(BASE_DIR, path, "tests.rs"))
        ):
            continue

        run(
            "cargo", "xbuild", "--target", "x86_64-linux-kernel-module",
            cwd=os.path.join(BASE_DIR, path),
            environ=dict(os.environ, RUST_TARGET_PATH=os.path.join(BASE_DIR, os.path.pardir))
        )
        run(
            "make", "-C", BASE_DIR, "TEST_DIR={}".format(path)
        )
        run("rustc", "--tests", os.path.join(BASE_DIR, path, "tests.rs"))
        # TODO: qemu
        run("./lib")


if __name__ == "__main__":
    main()

