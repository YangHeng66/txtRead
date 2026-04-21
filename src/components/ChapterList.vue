<template>
  <div class="list">
    <div class="header">目录（{{ chapters.length }}）</div>
    <div class="items">
      <div
        v-for="c in chapters"
        :key="c.idx"
        class="item"
        :class="{ active: c.idx === currentIdx }"
        @click="emit('select', c.idx)"
      >
        {{ c.title }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ChapterMeta } from '../api/tauri'
defineProps<{ chapters: ChapterMeta[]; currentIdx: number }>()
const emit = defineEmits<{ select: [idx: number] }>()
</script>

<style scoped>
.list { display: flex; flex-direction: column; height: 100%; border-right: 1px solid var(--border); background: var(--panel); }
.header { padding: 12px 14px; font-weight: 500; border-bottom: 1px solid var(--border); }
.items { flex: 1; overflow-y: auto; }
.item { padding: 10px 14px; cursor: pointer; border-bottom: 1px solid var(--border); font-size: 14px; color: var(--text); }
.item:hover { background: var(--bg); }
.item.active { background: var(--accent); color: #fff; }
</style>
