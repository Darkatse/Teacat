// three/drawCrystal.ts
import * as THREE from 'three';

// 定义晶体结构类型
interface CrystalStructure {
  lattice_vertices: [number, number, number][],
  atoms: Atom[];
}

interface Atom {
  name: string;
  x: number;
  y: number;
  z: number;
  radius: number;
  color: string;
}

// 全局变量，用于存储所有原子标签
let atomLabels: THREE.Sprite[] = [];

// 绘制晶格边界框
function drawLattice(scene: THREE.Scene, latticeVertices: [number, number, number][]) {

  // 定义晶格的边
  const edges = [
    [0, 1], [0, 3], [0, 4], [1, 2], [1, 5], [2, 3], [2, 6], [3, 7],
    [4, 5], [4, 7], [5, 6], [6, 7]
  ];

  const geometry = new THREE.BufferGeometry();
  const positions: number[] = []; 
  
  edges.forEach(([start, end]) => {
    positions.push(latticeVertices[start][0], latticeVertices[start][1], latticeVertices[start][2]);
    positions.push(latticeVertices[end][0], latticeVertices[end][1], latticeVertices[end][2]);
  });

  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));

  const material = new THREE.LineBasicMaterial({ color: 0x0000ff });
  const lattice = new THREE.LineSegments(geometry, material);
  scene.add(lattice);
}

// 在 drawCrystal 中调用 drawLattice
export function drawCrystal(scene: THREE.Scene, crystalData: CrystalStructure) {
  clearScene(scene);
  atomLabels = [];

  // 绘制晶格
  drawLattice(scene, crystalData.lattice_vertices);

  // 绘制原子
  crystalData.atoms.forEach((atom) => {
    const geometry = new THREE.SphereGeometry(atom.radius, 32, 32);
    const material = new THREE.MeshBasicMaterial({ color: atom.color });
    const sphere = new THREE.Mesh(geometry, material);
    sphere.position.set(atom.x, atom.y, atom.z);
    scene.add(sphere);

    // 添加原子标签
    const label = createAtomLabel(atom.name, atom.radius);
    label.position.set(atom.x, atom.y + atom.radius + 0.05, atom.z); // 标签位置略高于原子
    scene.add(label);
    atomLabels.push(label);
  });
}

// 动态创建原子标签，字体大小根据原子半径调整
function createAtomLabel(text: string, radius: number): THREE.Sprite {
  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d')!;

  // 动态调整字体大小，字体大小与原子半径相关
  const fontSize = Math.max(100, radius * 50); // 确保字体大小不小于24
  context.font = `${fontSize}px Arial`;
  context.fillStyle = 'black';
  context.fillText(text, 0, fontSize);

  const texture = new THREE.CanvasTexture(canvas);
  const material = new THREE.SpriteMaterial({ map: texture });
  const sprite = new THREE.Sprite(material);

  // 标签大小根据原子半径调整
  sprite.scale.set(radius * 1.5, radius * 0.75, 1); // 宽高与半径相关，确保自适应
  return sprite;
}

export function toggleAtomLabels() {
  if (atomLabels.length === 0) {
    console.warn("No atom labels to toggle.");
    return;
  }

  atomLabels.forEach((label) => {
    label.visible = !label.visible; // 切换标签的可见性
  });
}

// 清除场景中的所有对象
function clearScene(scene: THREE.Scene) {
  while (scene.children.length > 0) {
    const object = scene.children[0];
    scene.remove(object);
  }
}