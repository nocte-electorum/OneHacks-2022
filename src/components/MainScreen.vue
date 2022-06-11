<template>
	<div class="split left">
		<div class="centered">
			<h4>Template Left</h4>
			<p>
				Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
				Aenean euismod elementum nisi quis eleifend quam adipiscing vitae proin.
				Elit ut aliquam purus sit amet luctus venenatis lectus.
				Nibh nisl condimentum id venenatis a.
				Placerat orci nulla pellentesque dignissim enim sit amet venenatis.
				Vel eros donec ac odio tempor.
				Maecenas accumsan lacus vel facilisis volutpat est velit.
				Elit ut aliquam purus sit amet luctus venenatis lectus magna.
				Egestas erat imperdiet sed euismod nisi porta lorem mollis.
				Tellus mauris a diam maecenas sed enim.
				Amet purus gravida quis blandit turpis.
				Pellentesque id nibh tortor id.
				Nam libero justo laoreet sit amet cursus sit.
				Accumsan lacus vel facilisis volutpat.
				Ut lectus arcu bibendum at varius vel pharetra vel turpis.
				Proin libero nunc consequat interdum varius sit amet mattis vulputate.
				Mollis aliquam ut porttitor leo. Porttitor leo a diam sollicitudin tempor id.
				Imperdiet sed euismod nisi porta lorem mollis aliquam.
				Sed viverra tellus in hac habitasse platea dictumst.
				Facilisi nullam vehicula ipsum a arcu cursus.
				Pellentesque massa placerat duis ultricies.
				Velit dignissim sodales ut eu sem.
				Elementum pulvinar etiam non quam lacus suspendisse.
				Augue eget arcu dictum varius duis at consectetur lorem.
				Libero enim sed faucibus turpis in eu.
				Imperdiet nulla malesuada pellentesque elit eget gravida cum.
				Enim neque volutpat ac tincidunt vitae semper quis lectus nulla.
				Lorem ipsum dolor sit amet consectetur adipiscing elit ut.
				Amet volutpat consequat mauris nunc congue nisi vitae suscipit.
				Dictumst vestibulum rhoncus est pellentesque elit ullamcorper dignissim.
				Arcu risus quis varius quam quisque id.
				Quis eleifend quam adipiscing vitae.
				Donec ultrices tincidunt arcu non sodales neque.
				Eget dolor morbi non arcu risus quis.
				Leo urna molestie at elementum. Id volutpat lacus laoreet non curabitur gravida arcu ac.
				Vivamus arcu felis bibendum ut tristique. In egestas erat imperdiet sed euismod nisi.
				Est lorem ipsum dolor sit amet consectetur adipiscing.
				Vulputate dignissim suspendisse in est ante in nibh. Non nisi est sit amet facilisis.
				Fermentum odio eu feugiat pretium nibh ipsum consequat nisl vel.
				Tellus rutrum tellus pellentesque eu tincidunt tortor aliquam. Dignissim cras tincidunt lobortis feugiat.
				Velit euismod in pellentesque massa. Sed risus pretium quam vulputate dignissim.
				Sapien eget mi proin sed libero enim sed.
				Egestas erat imperdiet sed euismod nisi porta lorem mollis aliquam.
				Pellentesque massa placerat duis ultricies lacus sed turpis tincidunt id.
			</p>
		</div>
	</div>

	<div class="split right">
		<div class="centered">
			<h1>Template Right</h1>
		</div>
	</div>
</template>


<script>
export default {
	name: "MainScreen",  // Change this
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
	padding-top: 20px;
}

.left {
	left: 0;
	background-color: red;
	overflow-y: scroll;
}

.right {
	right: 0;
	width: 75%;
	background-color: blue;
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