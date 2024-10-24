<template>
    <div ref="threeContainer" style="width: 100%; height: 100%;"></div>
  </template>
  
  <script>
  import { onMounted, ref } from 'vue'
  import { initScene } from './three/scene'
  import { initCamera } from './three/camera'
  import { initRenderer } from './three/renderer'
  import { initControls } from './three/controls'
  import { drawCrystal, toggleAtomLabels } from './three/drawCrystal';
  import { listen } from '@tauri-apps/api/event';
  import * as THREE from 'three'
  
  export default {
    setup() {
      const threeContainer = ref(null);
      let scene, camera, renderer, controls;
  
      onMounted(() => {
        // Initialize Three.js components
        scene = initScene();
        camera = initCamera(window.innerWidth / window.innerHeight);
        renderer = initRenderer(threeContainer.value);
        controls = initControls(camera, renderer.domElement);
  
        // Handle window resizing
        window.addEventListener('resize', onWindowResize, false);

        // 监听后端发送的 cif-data 事件
        listen('cif-data', (event) => {
          const crystalData = event.payload;
          console.log('Received crystal data:', crystalData);
          drawCrystal(scene, crystalData);
        });

        // 切换原子标签的显示/隐藏
        listen('toggle-atom-labels', () => {
          console.log('Toggling atom labels');
          // 在这里切换原子标签的显示
          toggleAtomLabels();
        });

        // // 监听设置晶格重复数量的事件
        // listen('set-repeats', () => {
        //   console.log('Setting repeats for x/y/z');
        //   // 弹出滑块或对话框让用户设置 x/y/z 重复数量
        //   showRepeatSettings();
        // });
  
        // Start animation loop
        animate();
      });
  
      function animate() {
        requestAnimationFrame(animate);
        controls.update();
        renderer.render(scene, camera);
      }
  
      function onWindowResize() {
        camera.aspect = window.innerWidth / window.innerHeight;
        camera.updateProjectionMatrix();
        renderer.setSize(window.innerWidth, window.innerHeight);
      }
  
      return {
        threeContainer
      }
    }
  }
  </script>
  
  <style scoped>
  /* Add any specific styling here if needed */
  </style>