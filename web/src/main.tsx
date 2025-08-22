import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import {EntityRoutes} from "@/router/EntityRoutes.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <EntityRoutes/>
    <Toaster/>
  </StrictMode>,
)
