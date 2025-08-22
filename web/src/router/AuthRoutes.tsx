import type {RouteObject} from "react-router-dom";
import AuthLogin from "@/app/auth/login.tsx";
import AuthRegistry from "@/app/auth/register.tsx";
import AuthForget from "@/app/auth/forget.tsx";

export const AuthRoutes:RouteObject[] = [
    {
        path: "/auth",
        children: [
            {
                path: '',
                element: <AuthLogin/>
            },
            {
                path: "login",
                element: <AuthLogin/>
            },
            {
                path: "signup",
                element: <AuthRegistry/>
            },
            {
                path: "forget",
                element: <AuthForget/>
            }
        ]
    }
]