import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'


export default defineConfig(({mode}) => {
  const isDev = mode === 'dev'
  return ({
    plugins: [react()],
    build: {
      outDir: '../static',
      emptyOutDir: true,
      cssMinify: true,
      watch: isDev ? {} : null,
      sourcemap: isDev
    },
  })
})
