import type {RouteObject} from "react-router-dom";
import {UsersLayout} from "@/app/user/layout.tsx";
import OverviewPage from "@/app/user/overview.tsx";
import RepositoriesPage from "@/app/user/repository.tsx";
import ProductsPage from "@/app/user/product.tsx";
import FollowingPage from "@/app/user/following.tsx";
import StarsPage from "@/app/user/stars.tsx";

export const UserRoutes:RouteObject[] = [
    {
        path: "/:username",
        element: <UsersLayout/>,
        children: [
            {
                path: "",
                element: <OverviewPage/>
            },
            {
                path: "overview",
                element: <OverviewPage/>
            },
            {
                path: "repos",
                element: <RepositoriesPage/>
            },
            {
                path: "products",
                element: <ProductsPage/>
            },
            {
                path: "following",
                element: <FollowingPage/>
            },
            {
                path: "stars",
                element: <StarsPage/>
            }
        ]
    }
]