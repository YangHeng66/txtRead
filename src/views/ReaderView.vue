<template>
  <div class="reader" :class="{ collapsed: tocCollapsed }">
    <ChapterList
      :chapters="store.chapters"
      :current-idx="store.currentIdx"
      :collapsed="tocCollapsed"
      @select="onSelect"
      @toggle="tocCollapsed = !tocCollapsed"
    />
    <ReaderContent
      :chapter="store.current"
      :initial-ratio="store.scrollRatio"
      @scroll-change="onScroll"
    />
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import ChapterList from '../components/ChapterList.vue'
import ReaderContent from '../components/ReaderContent.vue'
import { useReaderStore } from '../stores/reader'

const route = useRoute()
const store = useReaderStore()
const tocCollapsed = ref(false)

async function boot() {
  const id = Number(route.params.id)
  if (Number.isFinite(id)) await store.openBook(id)
}

onMounted(boot)
watch(() => route.params.id, boot)
onBeforeUnmount(() => store.close())

async function onSelect(idx: number) {
  await store.loadChapter(idx)
  await store.saveProgress(0)
}

async function onScroll(ratio: number) {
  await store.saveProgress(ratio)
}
</script>

<style scoped>
.reader {
  display: grid;
  grid-template-columns: 260px 1fr;
  height: 100%;
  min-height: 0;
  transition: grid-template-columns 0.2s ease;
}
.reader.collapsed { grid-template-columns: 36px 1fr; }
</style>
