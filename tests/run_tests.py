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
    run("rustc", "--crate-type=rlib", os.path.join(BASE_DIR, "testlib.rs"))

    for path in os.listdir(BASE_DIR):
        if (
            not os.path.isdir(os.path.join(BASE_DIR, path)) or
            not os.path.exists(os.path.join(BASE_DIR, path, "tests.rs"))
        ):
            continue

        run(
            "cargo", "xbuild", "--target", "x86_64-linux-kernel-module",
            cwd=os.path.join(BASE_DIR, path),
            environ=dict(
                os.environ,
                RUST_TARGET_PATH=os.path.join(BASE_DIR, os.path.pardir),
                RUSTFLAGS="-Dwarnings",
                CARGO_TARGET_DIR=os.path.relpath(
                    os.path.join(BASE_DIR, "target"),
                    os.path.join(BASE_DIR, path)
                ),
            )
        )

        module = os.path.join(
            BASE_DIR,
            "target/x86_64-linux-kernel-module/debug/lib{}_tests.a".format(
                path
            )
        )
        library_archive, _ = os.path.splitext(os.path.basename(module))
        library_archive = os.path.join(library_archive, ".a")
        run(
            "make", "-C", BASE_DIR,
            "TEST_LIBRARY={}".format(
                os.path.join(
                    "target/x86_64-linux-kernel-module/debug/",
                    os.path.basename(module)
                )
            ),
            "TEST_LIBRARY_ARCHIVE={}".format(library_archive),
        )
        run(
            "rustc",
            "--test",
            "-Dwarnings",
            "--out-dir", os.path.join(BASE_DIR, path),
            os.path.join(BASE_DIR, path, "tests.rs"),
            "--extern", "kernel_module_tests=libtestlib.rlib",
        )
        # TODO: qemu
        run(
            os.path.join(BASE_DIR, path, "tests"), "--test-threads=1",
            environ=dict(
                os.environ,
                KERNEL_MODULE=os.path.join(BASE_DIR, "testmodule.ko")
            )
        )



if __name__ == "__main__":
    main()

