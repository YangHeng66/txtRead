<template>
  <div class="panel">
    <div class="bar">
      <n-input
        v-model:value="q"
        placeholder="输入关键词，在所有书内搜索"
        clearable
        @keyup.enter="run"
      />
      <n-button type="primary" @click="run" :loading="loading">搜索</n-button>
    </div>
    <div v-if="loading" class="info">搜索中…</div>
    <div v-else-if="queried && hits.length === 0" class="info">无结果</div>
    <div v-else class="hits">
      <div
        v-for="(h, i) in hits"
        :key="i"
        class="hit"
        @click="onOpen(h)"
      >
        <div class="meta">
          <span class="book">{{ h.book_title }}</span>
          <span class="chap">· {{ h.chapter_title }}</span>
        </div>
        <div class="snip" v-html="safe(h.snippet)"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { NInput, NButton } from 'naive-ui'
import { api, type SearchHit } from '../api/tauri'
import { useReaderStore } from '../stores/reader'

const router = useRouter()
const reader = useReaderStore()
const q = ref('')
const hits = ref<SearchHit[]>([])
const loading = ref(false)
const queried = ref(false)

async function run() {
  if (!q.value.trim()) return
  loading.value = true
  queried.value = true
  try {
    hits.value = await api.searchText(q.value.trim())
  } finally {
    loading.value = false
  }
}

async function onOpen(h: SearchHit) {
  await router.push(`/reader/${h.book_id}`)
  queueMicrotask(async () => {
    await reader.openBook(h.book_id)
    await reader.loadChapter(h.chapter_idx, 0)
    await reader.saveProgress(0)
  })
}

// snippet() from FTS5 carries <mark>…</mark>; escape everything else then restore mark tags.
function safe(html: string): string {
  return html
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/&lt;mark&gt;/g, '<mark>')
    .replace(/&lt;\/mark&gt;/g, '</mark>')
}
</script>

<style scoped>
.panel { padding: 16px; height: 100%; display: flex; flex-direction: column; gap: 12px; }
.bar { display: flex; gap: 8px; }
.info { color: var(--muted); text-align: center; padding: 40px 0; }
.hits { overflow-y: auto; display: flex; flex-direction: column; gap: 8px; }
.hit { padding: 12px; background: var(--panel); border: 1px solid var(--border); border-radius: 6px; cursor: pointer; }
.hit:hover { border-color: var(--accent); }
.meta { font-size: 13px; color: var(--muted); margin-bottom: 6px; }
.meta .book { color: var(--text); font-weight: 500; }
.snip :deep(mark) { background: var(--accent); color: #fff; padding: 0 2px; border-radius: 2px; }
</style>
