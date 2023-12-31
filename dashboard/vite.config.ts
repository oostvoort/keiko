import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import ImportMetaEnvPlugin from "@import-meta-env/unplugin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    ImportMetaEnvPlugin.vite({
      example: ".env.example"
    })
  ],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3000/',
        changeOrigin: true,
      },
    },
  },
  base: '/keiko'
})
