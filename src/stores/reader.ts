import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api, type Chapter, type ChapterMeta } from '@/api/tauri'

export const useReaderStore = defineStore('reader', () => {
  const bookId = ref<number | null>(null)
  const chapters = ref<ChapterMeta[]>([])
  const current = ref<Chapter | null>(null)
  const scrollRatio = ref(0)

  const currentIdx = computed(() => current.value?.idx ?? 0)

  async function openBook(id: number) {
    bookId.value = id
    chapters.value = await api.listChapters(id)
    const p = await api.getProgress(id)
    const idx = p?.chapter_idx ?? 0
    scrollRatio.value = p?.scroll_ratio ?? 0
    await loadChapter(idx, scrollRatio.value)
  }

  async function loadChapter(idx: number, ratio = scrollRatio.value) {
    if (bookId.value == null) return
    scrollRatio.value = ratio
    current.value = await api.getChapter(bookId.value, idx)
  }

  async function saveProgress(ratio: number) {
    if (bookId.value == null || current.value == null) return
    scrollRatio.value = ratio
    await api.saveProgress({
      book_id: bookId.value,
      chapter_idx: current.value.idx,
      scroll_ratio: ratio,
    })
  }

  function close() {
    bookId.value = null
    chapters.value = []
    current.value = null
    scrollRatio.value = 0
  }

  return { bookId, chapters, current, scrollRatio, currentIdx, openBook, loadChapter, saveProgress, close }
})
