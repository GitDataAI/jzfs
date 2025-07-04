import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import {GlobalHeader} from "taskbox/src";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      <GlobalHeader
          theme={"#e3e3e3"}
          create_menu={[
              {
                  url: '/',
                  name: "Create"
              },
              {
                  url: '/123',
                  name: "Create"
              },
          ]}
          user_avatar={"https://ss2.bdstatic.com/70cFvXSh_Q1YnxGkpoWK1HF6hhy/it/u=3315366793,2321372572&fm=253&gp=0.jpg"}
      />
  </StrictMode>,
)
