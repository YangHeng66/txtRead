<template>
  <n-space vertical size="large">
    <div>
      <div class="label">主题</div>
      <n-radio-group v-model:value="settings.theme">
        <n-radio value="light">浅色</n-radio>
        <n-radio value="dark">深色</n-radio>
      </n-radio-group>
    </div>
    <div>
      <div class="label">字号 ({{ settings.fontSize }}px)</div>
      <n-slider v-model:value="size" :min="0" :max="FONT_SIZES.length - 1" :marks="marks" />
    </div>
  </n-space>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NSpace, NRadioGroup, NRadio, NSlider } from 'naive-ui'
import { useSettingsStore, FONT_SIZES, type FontSize } from '../stores/settings'

const settings = useSettingsStore()
const size = computed({
  get: () => FONT_SIZES.indexOf(settings.fontSize),
  set: (v: number) => { settings.fontSize = FONT_SIZES[v] as FontSize },
})
const marks = Object.fromEntries(FONT_SIZES.map((s, i) => [i, `${s}`]))
</script>

<style scoped>
.label { font-weight: 500; margin-bottom: 6px; }
</style>
