<template>
  <div class="reader">
    <ChapterList
      :chapters="store.chapters"
      :current-idx="store.currentIdx"
      @select="onSelect"
    />
    <ReaderContent
      :chapter="store.current"
      :initial-ratio="store.scrollRatio"
      @scroll-change="onScroll"
    />
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import ChapterList from '../components/ChapterList.vue'
import ReaderContent from '../components/ReaderContent.vue'
import { useReaderStore } from '../stores/reader'

const route = useRoute()
const store = useReaderStore()

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
.reader { display: grid; grid-template-columns: 260px 1fr; height: 100%; }
</style>
