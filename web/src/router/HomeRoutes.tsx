import {type RouteObject} from "react-router-dom";
import {RepositoriesLayout} from "@/app/home/repositories/layout.tsx";
import {DashBoardLayout} from "@/app/home/dashboard/layout.tsx";
import {AiLayout} from "@/app/home/ai/layout.tsx";
import {DatasetLayout} from "@/app/home/dataset/layout.tsx";
import {MarketPlaceLayout} from "@/app/home/marketplace/layout.tsx";

export const HomeRoutes:RouteObject[] = [
    {
        path: "repositories",
        element: <RepositoriesLayout/>,
    },
    {
        path: "dashboard",
        element: <DashBoardLayout/>
    },
    {
        path: "ai",
        element: <AiLayout/>
    },
    {
        path: "dataset",
        element: <DatasetLayout/>
    },
    {
        path: "marketplace",
        element: <MarketPlaceLayout/>
    }
]