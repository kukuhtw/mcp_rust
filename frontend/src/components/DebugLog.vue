<!-- frontend/src/components/DebugLog.vue -->
<template>
  <details open>
    <summary>Debug Trace</summary>
    <pre>{{ pretty }}</pre>
  </details>
</template>

<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps<{ payload: any }>()
const pretty = computed(() => JSON.stringify(props.payload, null, 2))
</script>

<style scoped>
details { background:#f8fafc; border:1px solid #e5e7eb; border-radius:8px; padding:8px 10px; }
summary { cursor:pointer; font-weight:600; }
pre { margin:8px 0 0; white-space:pre-wrap; word-break:break-word; }
</style>
