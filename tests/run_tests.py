#!/usr/bin/env python

import os
import subprocess


BASE_DIR = os.path.realpath(os.path.dirname(__file__))


def run(*args, **kwargs):
    cwd = kwargs.pop("cwd", None)
    environ = kwargs.pop("environ", os.environ)
    assert not kwargs

    print("+ [running] {}".format(list(args)))
    subprocess.check_call(list(args), cwd=cwd, env=environ)


def main():
    for path in os.listdir(BASE_DIR):
        if (
            not os.path.isdir(os.path.join(BASE_DIR, path)) or
            not os.path.exists(os.path.join(BASE_DIR, path, "tests"))
        ):
            continue

        print("+ [{}]".format(path))
        run(
            "cargo", "xbuild",
            "--target",
            os.path.join(
                BASE_DIR, os.path.pardir, "x86_64-linux-kernel-module.json"
            ),
            cwd=os.path.join(BASE_DIR, path),
            environ=dict(
                os.environ,
                RUSTFLAGS="-Dwarnings",
                CARGO_TARGET_DIR=os.path.relpath(
                    os.path.join(BASE_DIR, "target"),
                    os.path.join(BASE_DIR, path)
                ),
                XBUILD_SYSROOT_PATH=os.path.join(BASE_DIR, "target-sysroot"),
            )
        )

        run(
            "make", "-C", BASE_DIR,
            "TEST_LIBRARY=target/x86_64-linux-kernel-module/debug/lib{}_tests.a".format(
                path.replace("-", "_")
            ),
        )
        # TODO: qemu
        run(
            "cargo", "test", "--", "--test-threads=1",
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
    main()
