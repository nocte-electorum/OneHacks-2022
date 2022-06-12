<template>
  <!-- <img alt="Vue logo" src="./assets/JitGuardian.png">
  <h1>JitGuardian</h1> -->
  <CreateScreen/>
</template>


<script>
import CreateScreen from './components/CreateScreen.vue';

  export default {
    name: "App",
	components: {
		CreateScreen
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
