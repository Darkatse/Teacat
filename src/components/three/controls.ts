// three/controls.ts
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

export function initControls(camera: THREE.PerspectiveCamera, domElement: HTMLElement): OrbitControls {
  const controls = new OrbitControls(camera, domElement);
  controls.enableDamping = true;  // 使旋转平滑
  controls.dampingFactor = 0.05;  // 阻尼系数
  controls.enableZoom = true;     // 启用缩放
  return controls;
}