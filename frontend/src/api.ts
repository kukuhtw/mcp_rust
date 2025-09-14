// frontend/src/api.ts

const BASE = (import.meta as any).env.VITE_BACKEND_BASE || ''

export type Range = { date_from?: string; date_to?: string; tz?: string }

export async function chat(text: string, range?: Range) {
  const r = await fetch(`/api/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text, ...range })
  })
  if (!r.ok) throw new Error(await r.text())
  return r.json()
}

// SSE GET (lebih simpel untuk EventSource)
export function streamChatGet(params: { text: string; tz?: string; date_from?: string; date_to?: string }) {
  const q = new URLSearchParams()
  q.set('text', params.text)
  if (params.tz) q.set('tz', params.tz)
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  const url = `${BASE}/api/chat/stream?${q.toString()}`
  return new EventSource(url)
}

// Optional settings endpoints (safe to call; backend may stub)
export async function getSettings() {
  const r = await fetch('/api/settings')
  if (!r.ok) return {}
  return r.json()
}
export async function saveSettings(payload: any) {
  const r = await fetch('/api/settings', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload)
  })
  return r.ok ? r.json() : {}
}
