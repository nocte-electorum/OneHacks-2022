

<template>
	<div class="split left">
		<div class="button-center">
			<button class="passwords"
				v-for="item in items"
				:key="item.id" @click="test">
				{{ item.message }}
			</button>
		</div>
	</div>

	<div class="split right">
		<div class="add">
			<button class="add-btn">
				<img src="../assets/Add.png">
			</button>
		</div>
		<div class="centered">
			<h1>Template Right</h1>
		</div>
	</div>
</template>


<script>
// eslint-disable-next-line no-unused-vars
import { invoke } from "@tauri-apps/api";

export default {
    data() {
        return {
			items: [
				{ message: "Playstation"}, 
				{ message: "D2L"},
				{ message: "Replit"},
				{ message: "Github"},
				{ message: "Youtube"},
				{ message: "Steam"},
				{ message: "Discord"},
				{ message: "Figma"},
				{ message: "Bank"},
				{ message: "Playstation"}, 
				{ message: "D2L"},
				{ message: "Replit"},
				{ message: "Github"},
				{ message: "Youtube"},
				{ message: "Steam"},
				{ message: "Discord"},
				{ message: "Figma"},
				{ message: "Bank"},
				{ message: "Playstation"}, 
				{ message: "D2L"},
				{ message: "Replit"},
				{ message: "Github"},
				{ message: "Youtube"},
				{ message: "Steam"},
				{ message: "Discord"},
				{ message: "Figma"},
				{ message: "Bank"},
			]
		};
    },
    name: "MainScreen",
    beforeCreate() {
        document.querySelector("body").setAttribute("style", "background:#3D3D3D");
    },
    beforeUnmount() {
        document.querySelector("body").setAttribute("style", "");
    },
    methods: {
		test() {
			invoke("log", { s: "edit mode" })
		}
    },
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

.split {
	height: 100%;
	width: 25%;
	position: fixed;
	z-index: 1;
	top: 0;
	overflow-x: hidden;
	padding-top: 10px;
}
.left {
	left: 0;
	background-color: #3A444F;
}

.right {
	right: 0;
	width: 75%;
	background-color: #4C5B69;
}
.button-center {
	margin: 0;
	margin-top: -10px;
	position: absolute;
	left: 50%;
	-ms-transform: translateX(-50%);
	transform: translateX(-50%);
}
.add {
	margin-top: -10px;
	background-color: #303030;
}
.passwords {
	background-color: #303030;
	display: block;
	align-content: center;
	border-top: none;
	border-left: none;
	border-right: none;
	border-bottom: 1px solid #5e5e5e;
	padding: 5px;
	width: 500px;
	color: #FFFFFF;
}
.passwords:hover {
	background-color: #1a1a1a;
}
.add-btn{
	margin-left: 75%;
}
::-webkit-scrollbar {
	width: 10px;
}

::-webkit-scrollbar-track {
	background: #303030;
}

::-webkit-scrollbar-thumb {
	background: #1a1a1a;
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
p {
	color: white;
}
/* padding format: top right bottom left */
</style>