import {createBrowserRouter, type RouteObject, RouterProvider} from "react-router-dom";
import {AuthRoutes} from "@/router/AuthRoutes.tsx";
import {AppLayout} from "@/app/layout.tsx";
import {InitRoutes} from "@/router/InitRoutes.tsx";
import {AboutRoutes, BasicAboutRoutes} from "@/router/AboutRoutes.tsx";
import {UserSettingRoutes} from "@/router/SettingRoutes.tsx";
import {RepoRoutes} from "@/router/RepoRoutes.tsx";
import {UserRoutes} from "@/router/UserRoutes.tsx";
import {useEffect} from "react";
import {useAuths} from "@/hooks/use-auths.tsx";
import {HomeRoutes} from "@/router/HomeRoutes.tsx";

export const EntityRoutes = () => {
    const auth = useAuths();
    useEffect(() => {
        auth.refresh_context();
    }, []);
    const Routes:RouteObject[] =
        [
            {
                path: "/",
                element: <AppLayout/>,
                children: [
                    ...HomeRoutes,
                    ...InitRoutes,
                ],
            },
            ...AuthRoutes,
            ...AboutRoutes,
            ...UserSettingRoutes,
            ...UserRoutes,
            ...RepoRoutes,
        ]
    const BasicRouter = [
        ...BasicAboutRoutes,
        ...AuthRoutes,
    ]
    return (
       <>
           {
               auth.isAuthenticated ? (
                   <RouterProvider router={createBrowserRouter(Routes)}></RouterProvider>
               ):(
                   <RouterProvider router={createBrowserRouter(BasicRouter)}></RouterProvider>
               )
           }
       </>
    )
}
