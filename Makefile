.PHONY : build build-linux build-osx build-win

PWD=$(shell pwd)

build:
	@echo 'building for local testing...' \
	&& cargo build

build-linux-from-osx:
	@echo 'building for Linux...' \
	&& rm -rf ${PWD}/builder-assets \
	&& ./scripts/prepare_build_linux_from_osx \
	&& rm -rf ${PWD}/build-result/linux \
	&& mkdir -p ${PWD}/build-result/linux \
	&& CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-unknown-linux-gnu-gcc" \
		TARGET_CFLAGS="-I ${PWD}/builder-assets/usr/include/x86_64-linux-gnu -isystem ${PWD}/builder-assets/usr/include" \
		LD_LIBRARY_PATH="${PWD}/builder-assets/usr/lib/x86_64-linux-gnu;${PWD}/builder-assets/lib/x86_64-linux-gnu" \
		OPENSSL_DIR="${PWD}/builder-assets/usr/" \
		OPENSSL_LIB_DIR="${PWD}/builder-assets/usr/lib/x86_64-linux-gnu/" \
		cargo build --target=x86_64-unknown-linux-gnu --target-dir=${PWD}/build-result/linux

build-linux:
	@echo 'building for Linux...' \
	&& rm -rf ${PWD}/build-result/linux \
	&& mkdir -p ${PWD}/build-result/linux \
	&& cargo build --target=x86_64-unknown-linux-gnu --target-dir=${PWD}/build-result/linux

build-osx:
	@echo 'building for OSX...' \
	&& rm -rf ${PWD}/build-result/osx \
	&& mkdir -p ${PWD}/build-result/osx \
	&& cargo build --target=x86_64-apple-darwin --target-dir=${PWD}/build-result/osx
# && ${PWD}/build-result/osx/x86_64-apple-darwin/debug/instaget -h 2>/dev/null; true

build-win:
	@echo 'building for Windows...' \
	&& rm -rf ${PWD}/build-result/win \
	&& mkdir -p ${PWD}/build-result/win \
	&& cargo build --target=x86_64-pc-windows-gnu --target-dir=${PWD}/build-result/win

clean:
	rm -rf ${PWD}/build-result/linux \
	&& rm -rf ${PWD}/build-result/osx \
	&& rm -rf ${PWD}/build-result/win \
	&& rm -rf builder-assets \
	&& rm -rf ${PWD}/target
