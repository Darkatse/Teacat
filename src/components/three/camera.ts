// three/camera.ts
import * as THREE from 'three';

export function initCamera(aspectRatio: number): THREE.PerspectiveCamera {
  const camera = new THREE.PerspectiveCamera(75, aspectRatio, 0.1, 1000);
  camera.position.z = 5;
  return camera;
}