package com.demo;

import android.content.Context;
import android.net.nsd.NsdManager;
import android.net.nsd.NsdServiceInfo;
import android.util.Log;

import com.facebook.react.bridge.Arguments;
import com.facebook.react.bridge.LifecycleEventListener;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.WritableMap;
import com.facebook.react.modules.core.DeviceEventManagerModule;

/**
 * Created by glinklater on 2018/01/18.
 */

public class NSDHelper implements LifecycleEventListener {
    private static final String SERVICE_TYPE = "_echo._tcp";
    private ReactApplicationContext reactContext;
    private NsdManager mNsdManager;
    private NsdManager.RegistrationListener mRegistrationListener;
    private NsdManager.DiscoveryListener mDiscoveryListener;

    private String serviceName;
    private int servicePort;

    public NSDHelper (ReactApplicationContext reactContext, String serviceName, int servicePort) {
        this.reactContext = reactContext;
        this.mNsdManager = (NsdManager) reactContext.getSystemService(Context.NSD_SERVICE);
        this.serviceName = serviceName;
        this.servicePort = servicePort;
    }

    private void initializeRegistrationListener () {
        mRegistrationListener = new NsdManager.RegistrationListener() {
            @Override
            public void onRegistrationFailed(NsdServiceInfo serviceInfo, int errorCode) {
                Log.e("ReactNative", String.format("Registration failed for %s with error code %d", serviceInfo.getServiceName(), errorCode));
            }

            @Override
            public void onUnregistrationFailed(NsdServiceInfo serviceInfo, int errorCode) {
                Log.e("ReactNative", String.format("Deregistration failed for %s with error code %d", serviceInfo.getServiceName(), errorCode));
            }

            @Override
            public void onServiceRegistered(NsdServiceInfo serviceInfo) {
                Log.d("ReactNative", String.format("Service registered %s", serviceInfo.toString()));
            }

            @Override
            public void onServiceUnregistered(NsdServiceInfo serviceInfo) {
                Log.d("ReactNative", String.format("Service deregistered %s", serviceInfo.getServiceName()));
            }
        };
    }

    private void initializeDiscoveryListener () {
        mDiscoveryListener = new NsdManager.DiscoveryListener() {
            @Override
            public void onStartDiscoveryFailed(String serviceType, int errorCode) {
                Log.e("ReactNative", String.format("Discovery start failed for %s with error code %d", serviceType, errorCode));
                mNsdManager.stopServiceDiscovery(this);
            }

            @Override
            public void onStopDiscoveryFailed(String serviceType, int errorCode) {
                Log.e("ReactNative", String.format("Discovery stop failed for %s with error code %d", serviceType, errorCode));
                mNsdManager.stopServiceDiscovery(this);
            }

            @Override
            public void onDiscoveryStarted(String serviceType) {
                Log.d("ReactNative", String.format("Discovery started for %s", serviceType));
            }

            @Override
            public void onDiscoveryStopped(String serviceType) {
                Log.d("ReactNative", String.format("Discovery stopped for %s", serviceType));
            }

            @Override
            public void onServiceFound(NsdServiceInfo serviceInfo) {
                Log.d("ReactNative", String.format("Found - %s", serviceInfo.toString()));
                WritableMap results = Arguments.createMap();

                results.putString("name", serviceInfo.getServiceName());
                results.putString("type", serviceInfo.getServiceType());
                results.putString("event", "found");

                reactContext
                        .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class)
                        .emit("mdns", results);

                resolveService(serviceInfo, null);
            }

            @Override
            public void onServiceLost(NsdServiceInfo serviceInfo) {
                WritableMap results = Arguments.createMap();

                results.putString("name", serviceInfo.getServiceName());
                results.putString("type", serviceInfo.getServiceType());
                results.putString("event", "lost");

                reactContext
                        .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class)
                        .emit("mdns", results);
            }
        };
    }

    private NsdManager.ResolveListener initializeResolveListener () {
        return new NsdManager.ResolveListener() {
            @Override
            public void onResolveFailed(NsdServiceInfo serviceInfo, int errorCode) {
                Log.e("ReactNative", String.format("Resolution failed for %s with error code %d", serviceInfo.getServiceName(), errorCode));
            }

            @Override
            public void onServiceResolved(NsdServiceInfo serviceInfo) {
                Log.d("ReactNative", "Resolved - " + serviceInfo);
                WritableMap results = Arguments.createMap();

                results.putString("name", serviceInfo.getServiceName());
                results.putString("type", serviceInfo.getServiceType());
                results.putString("host", serviceInfo.getHost().getCanonicalHostName());
                results.putInt("port", serviceInfo.getPort());
                results.putString("event", "resolved");

                reactContext
                        .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class)
                        .emit("mdns", results);
            }
        };
    }

    public void registerService (String serviceName, int port) {
        NsdServiceInfo serviceInfo = new NsdServiceInfo();

        serviceInfo.setServiceName(serviceName);
        serviceInfo.setServiceType(SERVICE_TYPE);
        serviceInfo.setPort(port);

        if (mRegistrationListener == null) {
            initializeRegistrationListener();
        }

        mNsdManager.registerService(serviceInfo, NsdManager.PROTOCOL_DNS_SD, mRegistrationListener);
    }

    public void discoverServices () {
        if (mDiscoveryListener == null) {
            initializeDiscoveryListener();
        }

        mNsdManager.discoverServices(SERVICE_TYPE, NsdManager.PROTOCOL_DNS_SD, mDiscoveryListener);
    }

    public void resolveService (NsdServiceInfo service, NsdManager.ResolveListener resolveListener) {
        if (resolveListener == null) {
            resolveListener = initializeResolveListener();
        }

        mNsdManager.resolveService(service, resolveListener);
    }

    public void teardown () {
        mNsdManager.unregisterService(mRegistrationListener);
        mNsdManager.stopServiceDiscovery(mDiscoveryListener);
    }

    @Override
    public void onHostResume () {
        this.registerService(serviceName, servicePort);
        this.discoverServices();
    }

    @Override
    public void onHostPause() {
        this.teardown();
    }

    @Override
    public void onHostDestroy() {
        this.teardown();
    }
}
