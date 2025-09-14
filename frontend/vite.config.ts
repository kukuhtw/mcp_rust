// frontend/vite.config.ts

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

const backendBase = process.env.VITE_BACKEND_BASE || 'http://localhost:8080'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: backendBase,
        changeOrigin: true
      }
    }
  }
})
