<script setup lang="ts">
// Stage 0 占位根组件；Task 6 会挂载 NotchView、Task 7 引入 SVG 刘海。
</script>

<template>
  <div class="app-root" data-testid="app-root">
    <slot />
  </div>
</template>

<style scoped>
.app-root {
  width: 100%;
  height: 100%;
}
</style>
