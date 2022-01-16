.PHONY : build build-linux build-osx build-win

PWD=$(shell pwd)
OPENSSL_DIR_OSX=/usr/local/Cellar/openssl@1.1/1.1.1m
build:
	echo 'building for local testing...' \
	&& cargo build

build-linux:
	echo 'building for Linux...' \
	&& rm -rf ${PWD}/build-result/linux \
	&& mkdir -p ${PWD}/build-result/linux \
	&& cargo build --target=x86_64-unknown-linux-gnu --target-dir=${PWD}/build-result/linux

build-osx:
	echo 'building for OSX...' \
	&& rm -rf ${PWD}/build-result/osx \
	&& mkdir -p ${PWD}/build-result/osx \
	&& cargo build --target=x86_64-apple-darwin --target-dir=${PWD}/build-result/osx
# && ${PWD}/build-result/osx/x86_64-apple-darwin/debug/instaget -h 2>/dev/null; true

build-win:
	echo 'building for Windows...' \
	&& rm -rf ${PWD}/build-result/win \
	&& mkdir -p ${PWD}/build-result/win \
	&& cargo build --target=x86_64-pc-windows-gnu --target-dir=${PWD}/build-result/win

clean:
	rm -rf ${PWD}/build-result/linux \
	&& rm -rf ${PWD}/build-result/osx \
	&& rm -rf ${PWD}/build-result/win
