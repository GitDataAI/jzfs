import type {RouteObject} from "react-router-dom";
import InitPage from "@/app/init/layout.tsx";
import CreateRepository from "@/app/init/repository.tsx";
import CreateProject from "@/app/init/project.tsx";
import CreateTeam from "@/app/init/team.tsx";
import CreateDataset from "@/app/init/dataset.tsx";

export const InitRoutes:RouteObject[] = [
    {
        path: "init",
        element: <InitPage/>,
    },
    {
        path: 'init/repository',
        element: <CreateRepository/>
    },
    {
        path: "init/project",
        element: <CreateProject/>
    },
    {
        path: "init/team",
        element: <CreateTeam/>
    },
    {
        path: "init/dataset",
        element: <CreateDataset/>
    }
]