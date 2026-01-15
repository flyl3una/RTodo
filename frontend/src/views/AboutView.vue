<template>
  <div class="about-view">
    <div class="placeholder">
      <h1>关于 RTodo</h1>
      <p>版本 0.1.0</p>
      <p>基于 Tauri + Vue 3 + Element Plus + TailwindCSS 构建</p>
      <p>跨平台待办事项管理应用</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 关于页面
</script>

<style scoped>
.about-view {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder {
  text-align: center;
}

.placeholder h1 {
  font-size: 2rem;
  margin-bottom: 1rem;
  color: var(--el-color-primary);
}

.placeholder p {
  color: var(--el-text-color-regular);
  margin-bottom: 0.5rem;
}
</style>
