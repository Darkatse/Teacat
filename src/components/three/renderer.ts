// three/renderer.ts
import * as THREE from 'three';

export function initRenderer(container: HTMLElement): THREE.WebGLRenderer {
  // add antialias in default
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.setClearColor(0xffffff, 1);
  container.appendChild(renderer.domElement);
  return renderer;
}