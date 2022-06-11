<template>
	<div class="login">  <!-- change this -->
		<h1>JitGuardian</h1>
		<img src="../assets/JitGuardian.png" width="64">
		<h4>User Name:</h4>
		<input placeholder="Username" id="userinput">
		<h4 style="margin-top: 25px;">Password:</h4>
		<input placeholder="Password" type="password" id="passinput">
		<br>
		<button @click="login">Log In</button>
		<p id="errortext">Error!  Wrong username or password.</p>
	</div>
</template>


<script>
import { invoke } from "@tauri-apps/api";

export default {
	name: "LoginScreen",  // Change this
	beforeCreate() {
		document.querySelector('body').setAttribute('style', 'background:#3D3D3D')
	},
	beforeUnmount() {
		document.querySelector('body').setAttribute('style', '')
	},
	methods: {
		async login() {
			let input1 = document.getElementById("userinput");
			let input2 = document.getElementById("passinput");
			let t = document.getElementById("errortext");
			await invoke("login", { username: input1.value, password: input2.value }).then(
				t.setAttribute('style', 'display: none;')
				// code to switch screens here
			).catch(() => {
				let t = document.getElementById("errortext");
				t.setAttribute('style', 'display: block;')
			})
		}
	}
}
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
/* padding format: top right bottom left */
@font-face {
  font-family: 'Source Sans Pro';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url(https://fonts.gstatic.com/s/sourcesanspro/v21/6xK3dSBYKcSV-LCoeQqfX1RYOo3qOK7l.woff2) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

html * {
	font-family: "Source Sans Pro";
}

h1 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
	font-size: 300%;
	margin-bottom: 5px;
}
h2 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
}
h3 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
}
h4 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
	margin-top: 50px;
	margin-bottom: 5px;
}
h5 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
}
body {
	background: #3D3D3D;
}
input {
	background-color: #303030;
	border: 0px;
	width: 25%;
	padding: 5px 0px 5px 0px;
	text-align: center;
	color: white;
}
button {
	margin-top: 60px;
	height: 52px;
	width: 192px;
	border: none;
	border-radius: 27px;
	font-family: "Source Sans Pro";
	font-size: large;
	color: #FFFFFF;
	background-color: #2E86DE;
}
button:hover {
	background-color: #266db4;
}
p {
	color: red;
	display: none;
}
/* padding format: top right bottom left */
</style>