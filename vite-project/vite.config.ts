import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { visualizer } from "rollup-plugin-visualizer";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(),visualizer({
    open: false
  })],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3080',
        changeOrigin: true,
      },
      "/git/" : {
        target: 'http://localhost:3080',
        changeOrigin: true,
      }
    },
  },
  build: {
    minify: "esbuild",
    rollupOptions:{
      output:{
        manualChunks(id) {
          if (id.includes('node_modules')) {
            return 'vendor';
          }
          return null;
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
