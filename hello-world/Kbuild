obj-m := helloworld.o
helloworld-objs := hello_world.rust.o

CARGO ?= cargo

export c_flags

$(src)/target/x86_64-linux-kernel/debug/libhello_world.a: cargo_will_determine_dependencies
	cd $(src); $(CARGO) build -Z build-std=core,alloc --target=x86_64-linux-kernel

.PHONY: cargo_will_determine_dependencies

%.rust.o: target/x86_64-linux-kernel/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<
