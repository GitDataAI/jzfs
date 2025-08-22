import type {RouteObject} from "react-router-dom";
import {RepoLayout} from "@/app/repo/layout.tsx";
import {BranchesPage} from "@/app/repo/branches.tsx";
import {CommitsPage} from "@/app/repo/commits.tsx";
import {TagsPage} from "@/app/repo/tags.tsx";
import {FilesPage} from "@/app/repo/file.tsx";
import {TreePage} from "@/app/repo/tree.tsx";

export const RepoRoutes:RouteObject[] = [
    {
        path: "/:owner/:repo",
        element: <RepoLayout/>,
        children: [
            {
                path: "",
                element: <FilesPage/>
            },
            {
                path: "branches",
                element: <BranchesPage/>
            },
            {
                path: "commits",
                element: <CommitsPage/>
            },
            {
                path: "tags",
                element: <TagsPage/>
            },
            {
                path: "files",
                element: <FilesPage/>
            },
            {
                path: "tree/*",
                element: <TreePage/>,
                loader: async ({params}) => {
                    return {
                        path: params["*"],
                    };
                }
            },
            {
                path: "*",
                element: <div>Repo</div>
            }
        ]
    }
]