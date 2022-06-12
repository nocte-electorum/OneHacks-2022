<template>
  <!-- <img alt="Vue logo" src="./assets/JitGuardian.png">
  <h1>JitGuardian</h1> -->
  <component :is="currentScreen"/>
</template>


<script>
import CreateScreen from './components/CreateScreen.vue';
import { invoke } from '@tauri-apps/api';
import LoginScreen from './components/LoginScreen.vue';
import MainScreen from './components/MainScreen.vue';

let currentScreen = "CreateScreen"

export default {
	provide: {
		getScreen: () => { return currentScreen },
		updateScreen: newscreen => {
			invoke("log", { s: `Updating screen to ${newscreen}` })
			currentScreen = newscreen
		}
	},
	data() {
		return {
			currentScreen: currentScreen,
			screens: ["CreateScreen", "LoginScreen", "MainScreen"]
		}
	},
	name: "App",
	components: {
		// eslint-disable-next-line vue/no-unused-components
		CreateScreen,
		// eslint-disable-next-line vue/no-unused-components
		LoginScreen,
		// eslint-disable-next-line vue/no-unused-components
		MainScreen
	}
}
</script>


<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
.titleColor {
  color: #222f3e
}
</style>
