package com.demo;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

public class MobileAppBridge extends ReactContextBaseJavaModule {
    static {
        System.loadLibrary("mobile_app");
    }

    @Override
    public String getName() {
        return "MobileAppBridge";
    }

    public MobileAppBridge(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @ReactMethod
    public void sayHelloWorld(String name, Promise promise) {
        promise.resolve(helloWorld(name));
    }

    @ReactMethod
    public void hashSHA256(String data, Promise promise) {
        promise.resolve(sha256(data));
    }

    @ReactMethod
    public void generateKey(Promise promise) {
        promise.resolve(ed25519GeneratePrivateKey());
    }

    @ReactMethod
    public void calculatePublicKey(String privateKey, Promise promise) {
        promise.resolve(ed25519GetPublicKey(privateKey));
    }

    @ReactMethod
    public void sign(String privateKey, String data, Promise promise) {
        String hashed = sha256(data);
        promise.resolve(ed25519Sign(privateKey, hashed));
    }

    @ReactMethod
    public void verify(String publicKey, String data, String signature, Promise promise) {
        String hashed = sha256(data);
        promise.resolve(ed25519Verify(publicKey, hashed, signature));
    }

    private static native String helloWorld(String seed);
    private static native String sha256(String data);
    private static native String ed25519GeneratePrivateKey();
    private static native String ed25519GetPublicKey(String key);
    private static native String ed25519Sign(String key, String data);
    private static native boolean ed25519Verify(String key, String msg, String sig);
}
