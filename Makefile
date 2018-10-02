
all:
	bootimage build

run:
	bootimage run -- -m 32 -serial stdio -no-reboot

debug:
	bootimage run -- -m 32 -serial stdio -d int -no-reboot


setup:
	cargo +nightly install bootimage
	cargo +nightly install cargo-xbuild

.PHONY: all
