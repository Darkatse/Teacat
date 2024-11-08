// three/camera.ts
import * as THREE from 'three';

// 初始化正交相机
export function initCamera(aspectRatio: number): THREE.OrthographicCamera {
  const frustumSize = 10; // 控制可视区域的大小，类似于缩放比例
  const halfWidth = frustumSize * aspectRatio / 2;
  const halfHeight = frustumSize / 2;

  // 创建正交相机，设置左右、上下、近、远平面的边界
  const camera = new THREE.OrthographicCamera(
    -halfWidth, // left
    halfWidth,  // right
    halfHeight, // top
    -halfHeight, // bottom
    -5,  // near plane
    1000  // far plane
  );

  // 设置相机位置
  camera.position.z = 5;
  camera.lookAt(0, 0, 0);

  return camera;
}


// 窗口调整大小时更新相机
export function updateCameraOnResize(camera: THREE.OrthographicCamera, aspectRatio: number) {
  const frustumSize = 10; // 保持与初始化时相同的 frustumSize
  const halfWidth = frustumSize * aspectRatio / 2;
  const halfHeight = frustumSize / 2;

  camera.left = -halfWidth;
  camera.right = halfWidth;
  camera.top = halfHeight;
  camera.bottom = -halfHeight;
  camera.updateProjectionMatrix(); // 更新相机的投影矩阵
}