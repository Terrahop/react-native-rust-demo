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
    const message = await MobileAppBridge.hashSHA256(greeting)

    self.setState({
      ...self.state,
      message
    })
  } catch (e) {
    console.log(e)
  }
}

export default class App extends Component<{}> {
  componentDidMount () {
    console.log(NativeModules.RustRN)
    displayHelloWorld(this)
  }

  state = {
    message: ''
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
