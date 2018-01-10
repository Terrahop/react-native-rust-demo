# Makefile

NAME = libmobile_app
LIB = $(NAME).a
SO = $(NAME).so
ARCHS_IOS = armv7-apple-ios armv7s-apple-ios aarch64-apple-ios
ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi arm-linux-androideabi i686-linux-android x86_64-linux-android
NDK_STANDALONE = $(PWD)/../NDK
ANDROID_DEST = ../android/app/src/main/jniLibs
IOS_DEST = ./target/universal/release

all: android ios

android: $(ARCHS_ANDROID)

ios: $(LIB)

clean:
	rm -R $(ANDROID_DEST)
	rm -R target

install-android: android
	mkdir -p $(ANDROID_DEST)
	mkdir -p $(ANDROID_DEST)/x86
	mkdir -p $(ANDROID_DEST)/x86_64
	mkdir -p $(ANDROID_DEST)/arm64-v8a
	mkdir -p $(ANDROID_DEST)/armeabi
	mkdir -p $(ANDROID_DEST)/armeabi-v7a

	cp ./target/i686-linux-android/release/$(SO) ../android/app/src/main/jniLibs/x86/$(SO)
	cp ./target/x86_64-linux-android/release/$(SO) ../android/app/src/main/jniLibs/x86_64/$(SO)
	cp ./target/aarch64-linux-android/release/$(SO) ../android/app/src/main/jniLibs/arm64-v8a/$(SO)
	cp ./target/arm-linux-androideabi/release/$(SO) ../android/app/src/main/jniLibs/armeabi/$(SO)
	cp ./target/armv7-linux-androideabi/release/$(SO) ../android/app/src/main/jniLibs/armeabi-v7a/$(SO)

aarch64-linux-android:
	PATH=$(PATH):$(NDK_STANDALONE)/arm64/bin \
	CC=$@-gcc \
	CXX=$@-g++ \
	cargo build --target $@ --release --lib

arm-linux-androideabi:
	PATH=$(PATH):$(NDK_STANDALONE)/arm/bin \
	CC=$@-gcc \
	CXX=$@-g++ \
	cargo build --target $@ --release --lib

armv7-linux-androideabi:
	PATH=$(PATH):$(NDK_STANDALONE)/arm/bin \
	CC=arm-linux-androideabi-gcc \
	CXX=arm-linux-androideabi-g++ \
	cargo build --target $@ --release --lib

i686-linux-android:
	PATH=$(PATH):$(NDK_STANDALONE)/x86/bin \
	CC=$@-gcc \
	CXX=$@-g++ \
	cargo build --target $@ --release --lib

x86_64-linux-android:
	PATH=$(PATH):$(NDK_STANDALONE)/x86_64/bin \
	CC=$@-gcc \
	CXX=$@-g++ \
	cargo build --target $@ --release --lib

.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	cargo build --target $@ --release --lib

$(LIB): $(ARCHS_IOS)
	mkdir -p $(IOS_DEST)
	lipo -create -output $(IOS_DEST)/$@ $(foreach arch,$(ARCHS_IOS),$(wildcard target/$(arch)/release/$(LIB)))
