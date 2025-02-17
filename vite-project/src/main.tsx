import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import {BrowserRouter} from "react-router-dom";
import {Toaster} from "@pheralb/toast";
import "./index.css"
import {HeroUIProvider} from "@heroui/system";

createRoot(document.getElementById('root')!).render(
  <HeroUIProvider>
      <BrowserRouter>
          <App />
          <Toaster position="top-right" />
      </BrowserRouter>
  </HeroUIProvider>,
)
