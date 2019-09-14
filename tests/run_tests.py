#!/usr/bin/env python

import os
import subprocess
import sys


BASE_DIR = os.path.realpath(os.path.dirname(__file__))


def run(*args, **kwargs):
    cwd = kwargs.pop("cwd", None)
    environ = kwargs.pop("environ", os.environ)
    assert not kwargs

    print("+ [running] {}".format(list(args)))
    subprocess.check_call(list(args), cwd=cwd, env=environ)


def main(argv):
    for path in os.listdir(BASE_DIR):
        if (
            not os.path.isdir(os.path.join(BASE_DIR, path)) or
            not os.path.exists(os.path.join(BASE_DIR, path, "tests"))
        ):
            continue

        print("+ [{}]".format(path))

        run(
            "make", "-C", BASE_DIR,
            "TEST_NAME={}_tests".format(path.replace("-", "_")),
            "TEST_PATH={}".format(path),
            "RUSTFLAGS=-Dwarnings",
        )
        # TODO: qemu
        run(
            "cargo", "test", "--no-default-features", "--", "--test-threads=1",
            cwd=os.path.join(BASE_DIR, path),
            environ=dict(
                os.environ,
                KERNEL_MODULE=os.path.join(BASE_DIR, "testmodule.ko"),
                RUSTFLAGS="-Dwarnings",
                CARGO_TARGET_DIR=os.path.relpath(
                    os.path.join(BASE_DIR, "target-test"),
                    os.path.join(BASE_DIR, path)
                ),
            )
        )


if __name__ == "__main__":
    main(sys.argv)
