<template>
  <div ref="scroller" class="scroller" @scroll="onScroll">
    <article v-if="chapter" class="article">
      <h1 class="h">{{ chapter.title }}</h1>
      <p v-for="(para, i) in paragraphs" :key="i" class="p">{{ para }}</p>
    </article>
    <div v-else class="empty">（未选中章节）</div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue'
import type { Chapter } from '../api/tauri'

const props = defineProps<{ chapter: Chapter | null; initialRatio: number }>()
const emit = defineEmits<{ 'scroll-change': [ratio: number] }>()

const scroller = ref<HTMLDivElement | null>(null)
const paragraphs = computed(() =>
  (props.chapter?.content ?? '').split(/\n{2,}/).map(s => s.trim()).filter(Boolean),
)

let lastEmit = 0
function onScroll(e: Event) {
  const el = e.target as HTMLDivElement
  const max = el.scrollHeight - el.clientHeight
  const ratio = max > 0 ? el.scrollTop / max : 0
  const now = Date.now()
  if (now - lastEmit > 300) {
    lastEmit = now
    emit('scroll-change', ratio)
  }
}

watch(() => props.chapter?.idx, async () => {
  await nextTick()
  if (!scroller.value) return
  const max = scroller.value.scrollHeight - scroller.value.clientHeight
  scroller.value.scrollTop = max * (props.initialRatio ?? 0)
}, { immediate: true })
</script>

<style scoped>
.scroller { height: 100%; overflow-y: auto; padding: 24px 32px; }
.article { max-width: 720px; margin: 0 auto; }
.h { font-size: 24px; margin: 0 0 24px; }
.p { font-size: var(--reader-font-size, 18px); line-height: 1.9; text-indent: 2em; margin: 0 0 14px; }
.empty { color: var(--muted); text-align: center; padding-top: 80px; }
</style>
