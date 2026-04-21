<template>
  <div class="card" @click="emit('open')">
    <div class="cover">{{ book.title.slice(0, 2) }}</div>
    <div class="meta">
      <div class="title">{{ book.title }}</div>
      <div class="sub">{{ book.author || '未知作者' }} · {{ book.format.toUpperCase() }}</div>
    </div>
    <n-button text @click.stop="emit('remove')" class="del">删除</n-button>
  </div>
</template>

<script setup lang="ts">
import { NButton } from 'naive-ui'
import type { Book } from '../api/tauri'
defineProps<{ book: Book }>()
const emit = defineEmits<{ open: []; remove: [] }>()
</script>

<style scoped>
.card { display: flex; gap: 12px; padding: 12px; border: 1px solid var(--border); border-radius: 8px; cursor: pointer; background: var(--panel); align-items: center; }
.card:hover { border-color: var(--accent); }
.cover { width: 56px; height: 76px; border-radius: 4px; background: var(--accent); color: #fff; display: grid; place-items: center; font-size: 20px; font-weight: 600; flex-shrink: 0; }
.meta { flex: 1; min-width: 0; }
.title { font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.sub { color: var(--muted); font-size: 13px; margin-top: 4px; }
.del { color: var(--muted); }
.del:hover { color: #dc2626; }
</style>
