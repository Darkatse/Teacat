<template>
  <div id="app">
    <three-scene></three-scene>
  </div>
</template>

<script>
import ThreeScene from './components/ThreeScene.vue'

export default {
  components: {
    ThreeScene
  }
}
</script>

<style>
#app {
  /* hidden scrollbar */
	position: fixed;
	width: 100%;
	height: 100%;
  overflow: hidden;
  background-color: #ffffff;
}
</style>