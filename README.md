# react-native-rust-demo
A cross platform demonstration of the execution of Rust code in React Native.

The application passes a string to the RN Native interface which then uses [ring](https://github.com/briansmith/ring) to create a sha256 hash of the input.

The hash is then displayed on the front-end of the app.

## Dependencies

This app depends on an installation of Android Studio for Android and Xcode for iOS.

Additionally the Android deployment requires the SDK for Android API 21 and the Android NDK to be installed using the Android Studio SDK manager.

### macOS Caveats

* Do NOT use brew/apt to install the Android SDK/NDK. Download the official version from the website.
* You may need to accept the Xcode license agreement in order to build: `sudo xcodebuild -license`

## Building

Both Android and iOS require that the NPM dependencies for React Native be installed locally and the `react-native-cli` globally.

```shell
$ npm install
$ npm -g install react-native-cli
```

### Rust

You must install the necessary rust targets in order to cross-compile for different architectures.

```shell
# iOS
$ rustup target add i386-apple-ios
$ rustup target add armv7-apple-ios
$ rustup target add armv7s-apple-ios
$ rustup target add aarch64-apple-ios
$ rustup target add x86_64-apple-ios

# Android
$ rustup target add i686-linux-android
$ rustup target add arm-linux-androideabi
$ rustup target add armv7-linux-androideabi
$ rustup target add aarch64-linux-android
$ rustup target add x86_64-linux-android
```

### Android

```shell
(./)     $ ./create-ndk-standalone.sh
(./)     $ cd rust
(./rust) $ make install-android
(./rust) $ cd ..
(./)     $ react-native run-android
```

### IOS

```shell
(./)     $ cd rust
(./rust) $ make ios
(./rust) $ cd ..

...?

(./)     $ react-native run-ios
```

TODO the `make ios` command results in the built iOS library being written to `./rust/target/universal/release/libmobile_app.a`. This needs to be used to create an iOS interface for RN.
