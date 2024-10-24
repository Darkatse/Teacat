<template>
    <div class="about-container">
      <h1>About This Application</h1>
      <p>Teacat stands for <strong>Tauri-Enhanced Adaptable Crystal Analysis Tools</strong>.</p>
      <p>This crystal visualization app was developed by <strong>Changjiang Wu</strong>.</p>
      <p>Version: <strong>0.1.0</strong></p>
      <p>
        For more information, please contact me at
        <a href="#" @click.prevent="openEmail">x.yangtze.river@gmail.com</a>.
      </p>
      

    <!-- 烟花效果的 Canvas -->
    <canvas v-if="showFireworks" ref="fireworksCanvas" class="fireworks-canvas"></canvas>
    </div>
</template>

<script setup>
// 这里可以添加更多的逻辑
import { open } from '@tauri-apps/plugin-shell';

// 打开 mailto 链接
async function openEmail() {
    await open('mailto:x.yangtze.river@gmail.com');
}

import { ref, onMounted, onBeforeUnmount } from 'vue';

const showFireworks = ref(false); // 控制烟花动画显示状态
let spacePressCount = 0; // 记录按下空格的次数
let fireworksInterval = null;

// 检测空格键事件
const handleKeyPress = (event) => {
    if (event.code === 'Space') {
        spacePressCount++;
        if (spacePressCount === 2) {
            triggerFireworks();
            spacePressCount = 0;
        }
    }
};

// 触发烟花动画
const triggerFireworks = () => {
    showFireworks.value = true;
    const canvas = document.querySelector('.fireworks-canvas');
    if (canvas) {
    startFireworks(canvas);
    }
};

// 启动烟花动画
const startFireworks = (canvas) => {
    const ctx = canvas.getContext('2d');
    const particles = [];
    const maxParticles = 100;

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // 粒子类
    class Particle {
    constructor() {
        this.x = canvas.width / 2;
        this.y = canvas.height / 2;
        this.size = Math.random() * 4 + 1;
        this.speedX = Math.random() * 3 - 1.5;
        this.speedY = Math.random() * 3 - 1.5;
        this.color = `hsl(${Math.random() * 360}, 100%, 50%)`;
    }

    update() {
        this.x += this.speedX;
        this.y += this.speedY;
    }

    draw() {
        ctx.fillStyle = this.color;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fill();
    }
    }

    // 创建粒子
    const createParticles = () => {
    for (let i = 0; i < maxParticles; i++) {
        particles.push(new Particle());
    }
    };

    // 更新和绘制粒子
    const updateParticles = () => {
    for (let i = 0; i < particles.length; i++) {
        particles[i].update();
        particles[i].draw();

        // 粒子超出边界时重新生成
        if (particles[i].x < 0 || particles[i].x > canvas.width || particles[i].y < 0 || particles[i].y > canvas.height) {
        particles.splice(i, 1);
        particles.push(new Particle());
        }
    }
    };

    // 动画循环
    const animate = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    updateParticles();
    fireworksInterval = requestAnimationFrame(animate);
    };

    createParticles();
    animate();

    // 10秒后停止烟花并隐藏 Canvas
    setTimeout(() => {
    cancelAnimationFrame(fireworksInterval);
    showFireworks.value = false;
    }, 10000);
};

// 绑定和解绑键盘事件
onMounted(() => {
  window.addEventListener('keydown', handleKeyPress);
});

onBeforeUnmount(() => {
    window.removeEventListener('keydown', handleKeyPress);
    if (fireworksInterval) {
        cancelAnimationFrame(fireworksInterval);
    }
});
</script>

<style scoped>
.about-container {
text-align: center;
padding: 10px;
}

h1 {
font-size: 24px;
margin-bottom: 10px;
}

p {
font-size: 14px;
margin-bottom: 8px;
}

.fireworks-canvas {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 1000;
  pointer-events: none;
}
</style>