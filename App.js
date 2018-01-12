/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 * @flow
 */

import React, { Component } from 'react'
import {
  Platform,
  StyleSheet,
  Text,
  View,
  NativeModules,
} from 'react-native'

async function displayHelloWorld (self) {
  const { MobileAppBridge } = NativeModules

  try {
    const name = Platform.select({
      ios: 'iOS',
      android: 'Android',
    })

    const greeting = await MobileAppBridge.sayHelloWorld(name)
    const hashed = await MobileAppBridge.hashSHA256(greeting)
    const privateKey = await MobileAppBridge.generateKey()
    console.log('PVT', privateKey)
    const publicKey = await MobileAppBridge.calculatePublicKey(privateKey)
    console.log('PUB', publicKey)
    const signature = await MobileAppBridge.sign(privateKey, name)
    console.log('SIG', signature)
    const reply = await MobileAppBridge.verify(publicKey, name, signature)
    console.log('RES_POS', reply)
    const neg_reply = await MobileAppBridge.verify(publicKey, "Invalid", signature)
    console.log('RES_NEG', reply)

    self.setState({
      ...self.state,
      message: `sha256(${greeting}): ${hashed}`,
      name,
      privateKey,
      publicKey,
      signature,
      neg_reply,
      reply
    })
  } catch (e) {
    console.log(e)
  }
}

export default class App extends Component<{}> {
  componentDidMount () {
    console.log(NativeModules.MobileAppBridge)
    displayHelloWorld(this)
  }

  state = {
    message: '',
    name: '',
    privateKey: '',
    publicKey: '',
    signature: '',
    neg_reply: '',
    reply: ''
  }

  render() {
    return (
      <View style={styles.container}>
        <Text style={styles.welcome}>
          Welcome to React Native!
        </Text>
        <Text style={styles.instructions}>
          {this.state.message}
        </Text>
        <Text style={styles.instructions}>
          Private Key: {this.state.privateKey}
        </Text>
        <Text style={styles.instructions}>
          Public Key: {this.state.publicKey}
        </Text>
        <Text style={styles.instructions}>
          Sign({this.state.name}): {this.state.signature}
        </Text>
        <Text style={styles.instructions}>
          Verification Result: {this.state.reply ? "Signature Verified" : "Failed to Verify"}
        </Text>
        <Text style={styles.instructions}>
          Negative Verification Result (should fail): {this.state.neg_reply ? "Signature Verified" : "Failed to Verify"}
        </Text>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#F5FCFF',
  },
  welcome: {
    fontSize: 20,
    textAlign: 'center',
    margin: 10,
  },
  instructions: {
    textAlign: 'center',
    color: '#333333',
    marginBottom: 5,
  },
})
