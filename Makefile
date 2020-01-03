ARCH=x86_64
TARGET=$(ARCH)-none-efi
CONFIG=debug
QEMU=qemu-system-$(ARCH)

TARGET_DIR=target/$(TARGET)/$(CONFIG)

BOOT_DIR=target/boot

all: build dist

build:
	cargo +nightly xbuild --target $(TARGET).json

	cd libuser && cargo +nightly xbuild --target ../$(TARGET).json
	cd binaries/init && cargo +nightly xbuild --release --target ../../$(TARGET).json

dist:
	mkdir -p $(BOOT_DIR)/EFI/BOOT $(BOOT_DIR)/EFI/Binaries

	cp $(TARGET_DIR)/uefi-kernel.efi $(BOOT_DIR)/EFI/BOOT/BOOTX64.EFI
	cp binaries/init/target/$(TARGET)/release/init.efi $(BOOT_DIR)/EFI/Binaries

	echo "EFI\BOOT\BOOTX64.EFI" > $(BOOT_DIR)/startup.nsh

run:
	qemu-system-x86_64 -nodefaults \
		 -vga std \
		 -machine q35 \
		 -m 128M \
		 -drive if=pflash,format=raw,readonly,file=OVMF_CODE.fd \
		 -drive if=pflash,format=raw,file=OVMF_VARS-1024x768.fd \
		 -drive format=raw,file=fat:rw:$(BOOT_DIR) \
		 -monitor vc:1024x768

.PHONY: all build
