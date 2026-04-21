<template>
  <div class="library">
    <div class="toolbar">
      <n-button type="primary" @click="onImport" :loading="importing">导入 txt / epub</n-button>
      <span class="count">共 {{ store.books.length }} 本</span>
    </div>
    <div v-if="store.books.length === 0 && !store.loading" class="empty">
      书库为空，点击上方按钮导入第一本书
    </div>
    <div class="grid">
      <BookCard
        v-for="b in store.books"
        :key="b.id"
        :book="b"
        @open="router.push(`/reader/${b.id}`)"
        @remove="onRemove(b.id)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { NButton, useMessage, useDialog } from 'naive-ui'
import BookCard from '../components/BookCard.vue'
import { useLibraryStore } from '../stores/library'

const store = useLibraryStore()
const router = useRouter()
const msg = useMessage()
const dialog = useDialog()
const importing = ref(false)

onMounted(() => store.refresh())

async function onImport() {
  importing.value = true
  try {
    const book = await store.importFromDialog()
    if (book) msg.success(`已导入：${book.title}`)
  } catch (e) {
    msg.error(`导入失败：${String(e)}`)
  } finally {
    importing.value = false
  }
}

function onRemove(id: number) {
  dialog.warning({
    title: '确认删除',
    content: '删除后无法恢复',
    positiveText: '删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      await store.remove(id)
      msg.info('已删除')
    },
  })
}
</script>

<style scoped>
.library { padding: 16px; height: 100%; overflow-y: auto; }
.toolbar { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
.count { color: var(--muted); font-size: 13px; }
.empty { text-align: center; color: var(--muted); padding: 80px 0; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 12px; }
</style>
