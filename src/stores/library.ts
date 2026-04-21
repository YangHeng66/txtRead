import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, type Book } from '@/api/tauri'
import { open } from '@tauri-apps/plugin-dialog'

export const useLibraryStore = defineStore('library', () => {
  const books = ref<Book[]>([])
  const loading = ref(false)

  async function refresh() {
    loading.value = true
    try { books.value = await api.listBooks() } finally { loading.value = false }
  }

  async function importFromDialog(): Promise<Book | null> {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Books', extensions: ['txt', 'epub'] }],
    })
    if (!selected || Array.isArray(selected)) return null
    const book = await api.importBook(selected as string)
    await refresh()
    return book
  }

  async function remove(id: number) {
    await api.deleteBook(id)
    await refresh()
  }

  return { books, loading, refresh, importFromDialog, remove }
})
