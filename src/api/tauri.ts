import { invoke } from '@tauri-apps/api/core'

export interface Book {
  id: number
  title: string
  author: string | null
  format: 'txt' | 'epub'
  source_path: string
  created_at: number
}

export interface ChapterMeta { idx: number; title: string }
export interface Chapter { idx: number; title: string; content: string }
export interface Progress { book_id: number; chapter_idx: number; scroll_ratio: number }
export interface SearchHit {
  book_id: number
  book_title: string
  chapter_idx: number
  chapter_title: string
  snippet: string
}

export const api = {
  importBook: (path: string) => invoke<Book>('import_book', { path }),
  listBooks: () => invoke<Book[]>('list_books'),
  deleteBook: (id: number) => invoke<void>('delete_book', { id }),
  listChapters: (bookId: number) => invoke<ChapterMeta[]>('list_chapters', { bookId }),
  getChapter: (bookId: number, idx: number) => invoke<Chapter>('get_chapter', { bookId, idx }),
  saveProgress: (progress: Progress) => invoke<void>('save_progress', { progress }),
  getProgress: (bookId: number) => invoke<Progress | null>('get_progress', { bookId }),
  setSetting: (key: string, value: string) => invoke<void>('set_setting', { key, value }),
  getSetting: (key: string) => invoke<string | null>('get_setting', { key }),
  searchText: (query: string, bookId?: number) =>
    invoke<SearchHit[]>('search_text', { query, bookId: bookId ?? null }),
}
