<template>
	<div class="login">  <!-- change this -->
		<h1>JitGuardian</h1>
		<img src="../assets/JitGuardian.png" width="64">
		<h4>User Name:</h4>
	</div>
</template>


<script>
export default {
	name: "LoginScreen",  // Change this
	beforeCreate() {
		document.querySelector('body').setAttribute('style', 'background:#3D3D3D')
	},
	beforeUnmount() {
		document.querySelector('body').setAttribute('style', '')
	}
}
</script>


<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
@font-face {
  font-family: 'Source Sans Pro';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url(https://fonts.gstatic.com/s/sourcesanspro/v21/6xK3dSBYKcSV-LCoeQqfX1RYOo3qOK7l.woff2) format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
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
}
h5 {
	color: #FFFFFF;
	font-family: "Source Sans Pro";
}
body {
	background: #3D3D3D;
}
</style>