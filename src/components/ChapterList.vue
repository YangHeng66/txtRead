<template>
  <div class="list" :class="{ collapsed }">
    <div class="header">
      <button class="toggle" :title="collapsed ? '展开目录' : '收起目录'" @click="emit('toggle')">
        {{ collapsed ? '›' : '‹' }}
      </button>
      <span v-if="!collapsed" class="title">目录（{{ chapters.length }}）</span>
    </div>
    <div v-if="!collapsed" class="items">
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
defineProps<{ chapters: ChapterMeta[]; currentIdx: number; collapsed: boolean }>()
const emit = defineEmits<{ select: [idx: number]; toggle: [] }>()
</script>

<style scoped>
.list {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  border-right: 1px solid var(--border);
  background: var(--panel);
  overflow: hidden;
}
.header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-bottom: 1px solid var(--border);
  font-weight: 500;
}
.list.collapsed .header { justify-content: center; padding: 8px 0; }
.toggle {
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--muted);
  font-size: 18px;
  line-height: 1;
  padding: 4px 8px;
  border-radius: 4px;
}
.toggle:hover { color: var(--accent); background: var(--bg); }
.title { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.items {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}
.item {
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  font-size: 14px;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item:hover { background: var(--bg); }
.item.active { background: var(--accent); color: #fff; }
</style>
