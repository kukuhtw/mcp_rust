<!-- frontend/src/App.vue -->
<template>
  <div class="app">
    <header class="bar">
      <h1>SMRT MCP PoC</h1>
      <button class="settings-btn" @click="showSettings = !showSettings">⚙️ Settings</button>
    </header>

    <div class="layout">
      <section class="left">
        <ChatPanel :streaming="streaming" />
      </section>
      <aside class="right" v-if="showSettings">
        <SettingsDrawer
          :initialModel="model"
          :initialTemperature="temperature"
          :initialTopP="topP"
          :initialSystemPrompt="systemPrompt"
          @save="saveSettings"
          @toggleStreaming="val => streaming = val"
          :streaming="streaming"
        />
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ChatPanel from './components/ChatPanel.vue'
import SettingsDrawer from './components/SettingsDrawer.vue'
import { getSettings, saveSettings as save } from './api'

const showSettings = ref(true)
const streaming = ref(true)

const model = ref('gpt-4o-mini')
const temperature = ref(0.2)
const topP = ref(0.9)
const systemPrompt = ref('You are an MCP intent router...')

async function load() {
  try {
    const s = await getSettings()
    if (s?.model) model.value = s.model
    if (typeof s?.temperature === 'number') temperature.value = s.temperature
    if (typeof s?.top_p === 'number') topP.value = s.top_p
    if (s?.system_prompt) systemPrompt.value = s.system_prompt
  } catch { /* optional: ignore if not implemented on backend */ }
}
load()

async function saveSettings(payload: any) {
  model.value = payload.model
  temperature.value = payload.temperature
  topP.value = payload.top_p
  systemPrompt.value = payload.system_prompt
  try { await save(payload) } catch { /* ignore for PoC */ }
}
</script>

<style scoped>
.app { font-family: ui-sans-serif, system-ui; color: #111; }
.bar { display:flex; align-items:center; justify-content:space-between; padding:12px 16px; border-bottom:1px solid #e5e7eb; }
.layout { display:grid; grid-template-columns: 1fr 360px; gap: 16px; height: calc(100vh - 58px); }
.left { height:100%; }
.right { border-left:1px solid #e5e7eb; padding:12px; overflow:auto; }
.settings-btn { background:#f3f4f6; border:1px solid #e5e7eb; padding:6px 10px; border-radius:8px; }
@media (max-width: 980px) {
  .layout { grid-template-columns: 1fr; }
  .right { border-left:0; border-top:1px solid #e5e7eb; }
}
</style>
