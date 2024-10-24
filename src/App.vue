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
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background-color: #ffffff;
}
</style>