<!-- frontend/src/components/ChatPanel.vue -->
<template>
  <div class="wrap">
    <!-- LEFT: Chat -->
    <div class="chat">
      <div class="msgs" ref="scrollEl">
        <div v-for="(m,i) in messages" :key="i" class="msg" :class="m.role">
          <strong>{{ m.role === 'user' ? 'You' : 'Bot' }}:</strong>
          <span>{{ m.text }}</span>
        </div>
      </div>

      <form class="input" @submit.prevent="onSend">
        <input
          v-model="text"
          placeholder="Ask something…"
          @keydown.enter.exact.prevent="onSend"
          @keydown.ctrl.enter.prevent="onSend"
          @keydown.meta.enter.prevent="onSend"
        />
        <div class="actions">
          <button type="button" v-if="streaming && es" @click="stopStream" class="secondary">Stop</button>
          <button type="submit" :disabled="busy">{{ busy ? 'Sending…' : 'Send' }}</button>
        </div>
      </form>
    </div>

    <!-- RIGHT: Debug Timeline -->
    <div class="debug">
      <div class="debug-head">
        <div class="badge">Debug Timeline</div>
        <div class="spacer" />
        <button class="ghost" @click="copyLog">Copy</button>
        <button class="ghost" @click="clearLog">Clear</button>
      </div>

      <div class="log" ref="logEl">
        <div v-for="(l, i) in timeline" :key="i" :class="['logline', l.level]">
          <span class="t">{{ l.time }}</span>
          <span class="k">{{ l.kind }}</span>
          <pre class="v" v-if="typeof l.data === 'string'">{{ l.data }}</pre>
          <pre class="v" v-else>{{ pretty(l.data) }}</pre>
        </div>
      </div>

      <!-- Router plan quick view -->
      <div v-if="lastPlan" class="plan card">
        <div class="muted">Router plan</div>
        <table class="kvs">
          <tr><td>intent</td><td><code>{{ lastPlan.intent }}</code></td></tr>
          <tr><td>endpoints</td><td><code>{{ lastPlan.endpoints.join(', ') }}</code></td></tr>
        </table>
        <details>
          <summary>params</summary>
          <pre>{{ pretty(lastPlan.params) }}</pre>
        </details>
      </div>

      <!-- Joined preview -->
      <div v-if="lastJoined" class="joined card">
        <div class="muted">Joined (preview)</div>
        <details open>
          <summary>results</summary>
          <pre style="max-height:220px; overflow:auto">{{ pretty(lastJoined) }}</pre>
        </details>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeUnmount, watch } from 'vue'
import { chat, streamChatGet } from '../api'

const props = defineProps<{ streaming: boolean }>()
const messages = ref<{role:'user'|'bot'; text:string}[]>([])
const text = ref('')
const busy = ref(false)

let es: EventSource | null = null
// expose es for template condition
// @ts-ignore
defineExpose({ es })

/** timeline items */
type TL = { time: string; kind: string; data: any; level?: 'ok'|'err'|'muted' }
const timeline = ref<TL[]>([])
const lastPlan = ref<{ intent:string; endpoints:string[]; params:Record<string,string> }|null>(null)
const lastJoined = ref<any|null>(null)

function now() {
  return new Date().toLocaleTimeString()
}
function log(kind: string, data?: any, level: TL['level'] = 'muted') {
  timeline.value.push({ time: now(), kind, data: data ?? '', level })
  requestAnimationFrame(() => {
    if (logEl.value) logEl.value.scrollTop = logEl.value.scrollHeight
  })
}
function pretty(v: any) { try { return JSON.stringify(v, null, 2) } catch { return String(v) } }
function clearLog() { timeline.value = []; lastPlan.value = null; lastJoined.value = null }

async function copyLog() {
  const txt = timeline.value.map(l => `[${l.time}] ${l.kind} — ${typeof l.data==='string'?l.data:pretty(l.data)}`).join('\n')
  try { await navigator.clipboard.writeText(txt); log('copied', 'timeline copied', 'ok') }
  catch { log('copy_failed', 'clipboard blocked', 'err') }
}

function stopStream() {
  if (es) { es.close(); es = null }
  busy.value = false
}

async function onSend() {
  if (!text.value.trim()) return
  const q = text.value.trim()
  messages.value.push({ role:'user', text:q })
  text.value = ''
  busy.value = true
  clearLog()

  try {
    if (props.streaming) {
      // === STREAM (SSE) ===
      stopStream() // close previous stream if any
      const qs = { text: q, tz: 'Asia/Singapore' }
      const qsStr = new URLSearchParams(qs as any).toString()
      log('request', `/api/chat/stream?${qsStr}`)

      es = streamChatGet(qs)

      // prepare bot bubble to append into
      messages.value.push({ role:'bot', text: '' })
      const idx = messages.value.length - 1

      // common/named events
      es.addEventListener('received', (ev: MessageEvent) => log('received', ev.data, 'muted'))

      es.addEventListener('llm_start', (ev: MessageEvent) => {
        const phase = ev.data === 'plan' ? 'plan' : 'answer'
        log('llm_start', phase, 'muted')
      })

      es.addEventListener('route_planned', (ev: MessageEvent) => {
        try {
          const plan = JSON.parse(ev.data)
          lastPlan.value = plan
          log('route_planned', plan, 'ok')
        } catch {
          log('route_planned', ev.data, 'err')
        }
      })

      es.addEventListener('fetch_progress', (ev: MessageEvent) => {
        try {
          const j = JSON.parse(ev.data)
          const lvl = j.status === 'ok' ? 'ok' : (j.status === 'error' ? 'err' : 'muted')
          log('fetch_progress', `${j.endpoint} → ${j.status}`, lvl as any)
        } catch {
          log('fetch_progress', ev.data, 'muted')
        }
      })

      es.addEventListener('joined', (ev: MessageEvent) => {
        try {
          const j = JSON.parse(ev.data)
          lastJoined.value = j
          log('joined', '(received)', 'ok')
        } catch {
          log('joined', ev.data, 'err')
        }
      })

      // token stream
      es.addEventListener('token', (ev: MessageEvent) => {
        messages.value[idx].text += ev.data
      })

      // some servers fall back to default event
      es.onmessage = (ev) => {
        if (!ev?.data) return
        messages.value[idx].text += ev.data
      }

      es.addEventListener('done', () => {
        log('done', 'done', 'ok')
        stopStream()
      })

      es.onerror = () => {
        messages.value[idx].text += (messages.value[idx].text ? '\n' : '') + '(stream error)'
        log('sse_error', 'connection lost', 'err')
        stopStream()
      }
    } else {
      // === NON-STREAM (POST JSON) ===
      log('request', '/api/chat')
      const res = await chat(q, { tz: 'Asia/Singapore' })
      messages.value.push({ role:'bot', text: res.reply ?? JSON.stringify(res) })
      busy.value = false
      log('done', 'ok', 'ok')
    }
  } catch (e: any) {
    messages.value.push({ role:'bot', text: `Error: ${e?.message || e}` })
    busy.value = false
    log('error', e?.message || String(e), 'err')
  }
}

onBeforeUnmount(stopStream)

// autoscroll
const scrollEl = ref<HTMLElement|null>(null)
const logEl = ref<HTMLElement|null>(null)
watch(messages, () => {
  requestAnimationFrame(() => {
    if (scrollEl.value) scrollEl.value.scrollTop = scrollEl.value.scrollHeight
  })
}, { deep: true })
</script>

<style scoped>
.wrap { display:grid; grid-template-columns: 1fr 380px; gap:16px; height:100%; }
.chat { height:100%; display:flex; flex-direction:column; }
.msgs { flex:1; overflow:auto; padding:12px; display:flex; flex-direction:column; gap:10px; background:#fafafa; border:1px solid #e5e7eb; border-radius:8px; }
.msg { padding:8px 10px; border-radius:10px; max-width: 80%; white-space: pre-wrap; }
.msg.user { align-self:flex-end; background:#e8f0ff; }
.msg.bot { align-self:flex-start; background:#f1f5f9; }
.input { display:flex; gap:8px; border-top:1px solid #e5e7eb; padding:10px 0; }
input { flex:1; padding:10px; border:1px solid #e5e7eb; border-radius:8px; }
.actions { display:flex; gap:8px; }
button { padding:10px 14px; border:1px solid #e5e7eb; border-radius:8px; background:#111; color:#fff; cursor:pointer; }
button.secondary { background:#f3f4f6; color:#111; }
button.ghost { background:#fff; color:#111; }
button[disabled] { opacity:.6; cursor:not-allowed; }

.debug { display:flex; flex-direction:column; gap:10px; }
.debug-head { display:flex; align-items:center; gap:8px; }
.spacer { flex:1; }
.badge { display:inline-block; padding:2px 8px; border-radius:999px; background:#eef2ff; color:#3730a3; font-size:12px; }
.log { border:1px solid #e5e7eb; border-radius:8px; padding:8px; height:260px; overflow:auto; background:#fff; }
.logline { display:grid; grid-template-columns: 82px 130px 1fr; gap:8px; align-items:start; font-size:12px; }
.logline .t { color:#6b7280; }
.logline .k { font-weight:600; color:#111827; }
.logline.ok .k { color:#047857; }
.logline.err .k { color:#b91c1c; }
.logline .v { margin:0; white-space:pre-wrap; }
.card { border:1px solid #e5e7eb; border-radius:8px; padding:8px; background:#fff; }
.muted { color:#6b7280; font-size:12px; margin-bottom:4px; }
.kvs { width:100%; font-size:12px; }
.kvs td { padding:2px 6px; vertical-align:top; }
</style>
