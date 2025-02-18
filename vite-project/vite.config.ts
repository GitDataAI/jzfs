import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3080',
        changeOrigin: true,
      },
    },
  },
  build: {
    minify: "esbuild",
    rollupOptions:{
      output:{
        manualChunks(id) {
          if (id.includes('node_modules')) {
            return "vers"
          }
        }
      }
    }
  },
  environments: {

  },
  resolve: {
    alias: {
      '@': '/src',
    },
  },
})
