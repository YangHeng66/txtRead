import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { api } from '@/api/tauri'

export type Theme = 'light' | 'dark'
export const FONT_SIZES = [16, 18, 20, 22, 24] as const
export type FontSize = typeof FONT_SIZES[number]

export const useSettingsStore = defineStore('settings', () => {
  const theme = ref<Theme>('light')
  const fontSize = ref<FontSize>(18)
  const ready = ref(false)

  async function load() {
    const t = (await api.getSetting('theme')) as Theme | null
    const f = await api.getSetting('font_size')
    if (t === 'light' || t === 'dark') theme.value = t
    const parsed = f ? parseInt(f, 10) : NaN
    if ((FONT_SIZES as readonly number[]).includes(parsed)) fontSize.value = parsed as FontSize
    ready.value = true
    applyTheme()
  }

  function applyTheme() {
    document.documentElement.dataset.theme = theme.value
    document.documentElement.style.setProperty('--reader-font-size', `${fontSize.value}px`)
  }

  watch([theme, fontSize], async () => {
    if (!ready.value) return
    applyTheme()
    await api.setSetting('theme', theme.value)
    await api.setSetting('font_size', String(fontSize.value))
  })

  return { theme, fontSize, ready, load }
})
