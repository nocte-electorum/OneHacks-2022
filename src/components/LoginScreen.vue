<template>
	<div :class="{ shake: disabled }" class="login">  <!-- change this -->
		<h1>JitGuardian</h1>
		<img src="../assets/JitGuardian.png" width="64">
		<h4>User Name:</h4>
		<input placeholder="Username" id="userinput">
		<h4 style="margin-top: 25px;">Password:</h4>
		<input placeholder="Password" type="password" id="passinput">
		<br>
		<button @click="login">Log In</button>
		<p id="errortext" class="errortext">Error!  Wrong username or password.</p>
		<p id="nonetext" class="errortext">Error!  Please enter a username and password.</p>
		<p id="successtext" class="successtext">Successful login!  Rediriecting...</p>
	</div>
</template>


<script>
import { invoke } from "@tauri-apps/api";

export default {
	data() {
		return {
			disabled: false  // disables false  
		}
	},
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
			let t2 = document.getElementById("nonetext");

			if (input1.value.trim() == "" && input2.value.trim() == "") {
				t2.setAttribute('style', 'display: block;')
				t.setAttribute('style', 'display: none;')
				this.errorDisabled()
			} else {
				await invoke("login", { username: input1.value, password: input2.value }).then(
					t.setAttribute('style', 'display: none;')
					// code to switch screens here
				).catch(() => {
					t.setAttribute('style', 'display: block;')
					t2.setAttribute('style', 'display: none;')
					this.errorDisabled()
				})
			}
		},
		errorDisabled() {
			this.disabled = true
			setTimeout(() => {
				this.disabled = false
			}, 1500)
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
	cursor: pointer;
}
.errortext {
	color: red;
	display: none;
}
.successtext {
	color: green;
	display: none;
}
/* padding format: top right bottom left */

.shake {
  animation: shake 0.82s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
  transform: translate3d(0, 0, 0);
}

@keyframes shake {
  10%, 90% {
    transform: translate3d(-1px, 0, 0);
  }

  20%, 80% {
    transform: translate3d(2px, 0, 0);
  }

  30%, 50%, 70% {
    transform: translate3d(-4px, 0, 0);
  }

  40%, 60% {
    transform: translate3d(4px, 0, 0);
  }
}
</style>