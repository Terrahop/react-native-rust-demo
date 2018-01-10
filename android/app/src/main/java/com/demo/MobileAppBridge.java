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

    private static native String helloWorld(String seed);
    private static native String sha256(String data);
}
