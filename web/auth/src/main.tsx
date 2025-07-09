import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import {AuthLayout} from "./layout";
import "./module.css"
import {LoginPage} from "./P-Login";
import {RegisterPage} from "./P-Register";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      <RouterProvider router={createBrowserRouter([
          {
              path: '/auth',
              element: <AuthLayout/>,
              children: [
                  {
                      path: '/',
                      element: <LoginPage/>
                  },
                  {
                      path: 'login',
                      element: <LoginPage/>
                  },
                  {
                      path: 'register',
                      element: <RegisterPage/>
                  }
              ]
          }
      ])}/>
  </StrictMode>,
)


