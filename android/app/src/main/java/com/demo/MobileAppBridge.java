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
    public void verifyWithEd25519(String data, Promise promise){ //proof of concept
        String privKey = ed25519GeneratePrivateKey("");
        String pubKey = ed25519GetPublicKey(privKey);
        String sig = ed25519Sign(privKey, data);
        String verified = ed25519Verify(pubKey,data,sig);
        String response = "";
        if(verified.equals("1")){
           response = "Verified, Hello from rust!";
        }
        else if (verified.equals("0")){
           response = "Key verification failure!";
        }
        promise.resolve(response);
    }

    private static native String helloWorld(String seed);
    private static native String sha256(String data);
    private static native String ed25519GeneratePrivateKey(String data);
    private static native String ed25519GetPublicKey(String key);
    private static native String ed25519Sign(String key, String data);
    private static native String ed25519Verify(String key, String msg, String sig);
}
