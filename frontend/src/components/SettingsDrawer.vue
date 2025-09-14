<!-- frontend/src/components/SettingsDrawer.vue -->
<template>
  <div class="drawer">
    <h3>Settings</h3>

    <div class="row">
      <label>Streaming</label>
      <label class="switch">
        <input type="checkbox" :checked="streaming" @change="toggleStreaming" />
        <span></span>
      </label>
    </div>

    <div class="row">
      <label>Model</label>
      <input v-model="model" />
    </div>

    <div class="row">
      <label>Temperature</label>
      <input type="number" step="0.1" v-model.number="temperature" />
    </div>

    <div class="row">
      <label>Top P</label>
      <input type="number" step="0.1" v-model.number="top_p" />
    </div>

   

    <button class="save" @click="onSave">Save</button>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'

const props = defineProps<{
  initialModel: string
  initialTemperature: number
  initialTopP: number
  initialSystemPrompt: string
  streaming: boolean
}>()
const emit = defineEmits<{
  (e: 'save', payload: any): void
  (e: 'toggleStreaming', val: boolean): void
}>()

const model = ref(props.initialModel)
const temperature = ref(props.initialTemperature)
const top_p = ref(props.initialTopP)
const system_prompt = ref(props.initialSystemPrompt)

watchEffect(() => {
  model.value = props.initialModel
  temperature.value = props.initialTemperature
  top_p.value = props.initialTopP
  system_prompt.value = props.initialSystemPrompt
})

function onSave() {
  emit('save', {
    model: model.value,
    temperature: temperature.value,
    top_p: top_p.value,
    system_prompt: system_prompt.value
  })
}
function toggleStreaming(e: Event) {
  const checked = (e.target as HTMLInputElement).checked
  emit('toggleStreaming', checked)
}
</script>

<style scoped>
.drawer { display:flex; flex-direction:column; gap:12px; }
h3 { margin: 4px 0 8px; }
.row { display:flex; flex-direction:column; gap:6px; }
input, textarea { padding:8px 10px; border:1px solid #e5e7eb; border-radius:8px; }
.save { align-self:flex-start; padding:8px 12px; border:1px solid #e5e7eb; border-radius:8px; background:#111; color:white; }

.switch { position: relative; width: 44px; height: 24px; display:inline-block; }
.switch input { display:none; }
.switch span { position:absolute; cursor:pointer; inset:0; background:#e5e7eb; border-radius:999px; transition:.2s; }
.switch span:after { content:''; position:absolute; width:20px; height:20px; left:2px; top:2px; background:#fff; border-radius:50%; transition:.2s; box-shadow: 0 1px 3px rgba(0,0,0,.2); }
.switch input:checked + span { background:#111; }
.switch input:checked + span:after { transform: translateX(20px); }
</style>
