import {SidebarInset, SidebarProvider} from "@/components/ui/sidebar.tsx";
import {AppSidebar} from "@/components/shell/app-sidebar.tsx";
import {DefaultRepoNavItem} from "@/data/system/homeNav.tsx";
import {NavLink, Outlet, useParams} from "react-router-dom";
import {Avatar, AvatarFallback, AvatarImage} from "@/components/ui/avatar.tsx";
import {Badge} from "@/components/ui/badge.tsx";
import { Button } from "@/components/ui/button.tsx";
import {Eye, GitFork, Star} from "lucide-react";
import {useEffect, useState} from "react";
import {type RepoData, RepoDataContext, useRepoData} from "@/hooks/use-repo-data.tsx";

export const RepoLayout = () => {
    const {owner, repo} = useParams();
    const [context, setContext] = useState<RepoData | undefined>();
    const repo_data = useRepoData();
    useEffect(() => {
        if (owner && repo) {
            repo_data.fetchRepoData(owner, repo)
                .then(res=>{
                    if (res) {
                        setContext(res);
                    }
                })
        }
    }, []);
    return(
        <SidebarProvider>
            {
                context && typeof owner === "string" && typeof repo === "string" ? (
                    <>
                        <AppSidebar main={DefaultRepoNavItem(owner, repo, context.is_owner)}/>
                        <SidebarInset>
                            <header className="flex shrink-0 items-center gap-2 px-4 mt-1">
                                <div className="container mx-auto px-4 py-4">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-4">
                                            <Avatar className="h-8 w-8">
                                                <AvatarImage src="/github-logo.png" />
                                                <AvatarFallback>GH</AvatarFallback>
                                            </Avatar>
                                            <div className="flex items-center gap-2">
                                                <NavLink to={`/${owner}`} className="hover:underline cursor-pointer">
                                                    {owner}
                                                </NavLink>
                                                <span className="text-muted-foreground">/</span>
                                                <span className="font-semibold text-lg">{repo}</span>
                                                <Badge variant="secondary" className="ml-2">
                                                    {
                                                        context.model.is_private ? (
                                                            <Badge variant="destructive">Private</Badge>
                                                        ) : (
                                                            <Badge variant="secondary">Public</Badge>
                                                        )
                                                    }
                                                </Badge>
                                            </div>
                                        </div>
                                        <div className="flex items-center gap-2">
                                            <Button variant="outline" size="sm">
                                                <Eye className="h-4 w-4 mr-1" />
                                                Watch
                                                <Badge variant="secondary" className="ml-1">
                                                    0
                                                </Badge>
                                            </Button>
                                            <Button variant="outline" size="sm">
                                                <Star className="h-4 w-4 mr-1" />
                                                Star
                                                <Badge variant="secondary" className="ml-1">
                                                    0
                                                </Badge>
                                            </Button>
                                            <Button variant="outline" size="sm">
                                                <GitFork className="h-4 w-4 mr-1" />
                                                Fork
                                                <Badge variant="secondary" className="ml-1">
                                                    0
                                                </Badge>
                                            </Button>
                                        </div>
                                    </div>

                                    <div className="mt-4">
                                        <p className="text-muted-foreground">
                                            {context.model.description}
                                        </p>
                                    </div>
                                </div>
                            </header>
                            <div className="container mx-auto px-4 py-4">
                                <RepoDataContext value={context}>
                                    <Outlet/>
                                </RepoDataContext>
                            </div>
                        </SidebarInset>
                    </>
                ):(
                    <>
                        <AppSidebar/>
                        <SidebarInset>
                            <div className="container mx-auto px-4 py-4">
                                404
                            </div>
                        </SidebarInset>
                    </>
                )
            }
        </SidebarProvider>
    )
}