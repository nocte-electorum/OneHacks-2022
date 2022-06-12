<template>
  <component :is="currentScreen"/>
</template>


<script>
import CreateScreen from './components/CreateScreen.vue';
// import { invoke } from '@tauri-apps/api';
import LoginScreen from './components/LoginScreen.vue';
import MainScreen from './components/MainScreen.vue';

// eslint-disable-next-line no-unused-vars
// let currentScreen = "CreateScreen"

export default {
	provide() {
		return {
			getScreen: () => { return this.currentScreen },
			updateScreen: newscreen => {
				this.currentScreen = newscreen
			}
		}
	},
	data() {
		this.currentScreen = "CreateScreen"
		return {
			currentScreen: this.currentScreen,
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
