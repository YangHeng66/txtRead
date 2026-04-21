<template>
  <n-config-provider :theme="nTheme">
    <n-message-provider>
      <n-dialog-provider>
        <div class="shell">
          <nav class="navbar">
            <router-link to="/library">书库</router-link>
            <router-link to="/search">搜索</router-link>
            <n-button text @click="showSettings = true">设置</n-button>
          </nav>
          <main class="main">
            <router-view />
          </main>
          <n-drawer v-model:show="showSettings" :width="320">
            <n-drawer-content title="设置">
              <SettingsPanel />
            </n-drawer-content>
          </n-drawer>
        </div>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import {
  darkTheme,
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NButton,
  NDrawer,
  NDrawerContent,
} from 'naive-ui'
import SettingsPanel from './components/SettingsPanel.vue'
import { useSettingsStore } from './stores/settings'

const settings = useSettingsStore()
const showSettings = ref(false)
const nTheme = computed(() => (settings.theme === 'dark' ? darkTheme : null))

onMounted(() => settings.load())
</script>

<style scoped>
.shell { display: flex; flex-direction: column; height: 100vh; }
.navbar { display: flex; gap: 16px; padding: 10px 16px; border-bottom: 1px solid var(--border); align-items: center; }
.navbar a { color: var(--text); text-decoration: none; padding: 4px 8px; border-radius: 4px; }
.navbar a.router-link-active { background: var(--accent); color: #fff; }
.main { flex: 1; overflow: hidden; }
</style>
