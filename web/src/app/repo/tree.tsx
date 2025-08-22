"use client"

import {Card, CardContent, CardHeader} from "@/components/ui/card"
import {ChevronRight, File, Folder, GitBranch} from "lucide-react"
import {NavLink, useLoaderData, useNavigate} from "react-router-dom";
import {useContext, useEffect, useState} from "react";
import {type BranchInfo, RepoDataContext, type TreeItemLastCommit, useRepoData} from "@/hooks/use-repo-data.tsx";
import {formatRelativeTime} from "@/lib/utils.ts";
import {Button} from "@/components/ui/button.tsx";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";


export function TreePage() {
    const context = useContext(RepoDataContext);
    const data = useRepoData();
    const [SelectedBranch, setSelectedBranch] = useState<BranchInfo | null>(null);
    const [Branches, setBranches] = useState<BranchInfo[]>([]);
    const [Tree, setTree] = useState<TreeItemLastCommit[]>([]);
    const props: {
        path: string
    } = useLoaderData();
    useEffect(() => {
        if (context) {
            data.fetchRepoBranches(context.owner.username, context.model.repo_name)
                .then(res=>{
                    if (res) {
                        setBranches(res);
                        const page_branch = res.find((b)=>b.branch.ref_name === context.model.default_head) || res[0];
                        setSelectedBranch(page_branch);
                        data.fetchRepoTree(context.owner.username, context.model.repo_name, page_branch.branch.ref_name, undefined, ConPath(props.path))
                            .then(res=>{
                                if (res) {
                                    res.sort((a, b)=>{
                                        if (a.item.kind === "Tree" && b.item.kind !== "Tree") {
                                            return -1;
                                        } else if (a.item.kind !== "Tree" && b.item.kind === "Tree") {
                                            return 1;
                                        } else {
                                            return a.item.name.localeCompare(b.item.name);
                                        }
                                    })
                                    setTree(res);
                                }
                            })
                    }
                })
        }
    }, []);
    useEffect(()=>{
        if (SelectedBranch && context) {
            data.fetchRepoTree(context.owner.username, context.model.repo_name, SelectedBranch.branch.ref_name, undefined, ConPath(props.path))
                .then(res=>{
                    if (res) {
                        res.sort((a, b)=>{
                            if (a.item.kind === "Tree" && b.item.kind !== "Tree") {
                                return -1;
                            } else if (a.item.kind !== "Tree" && b.item.kind === "Tree") {
                                return 1;
                            } else {
                                return a.item.name.localeCompare(b.item.name);
                            }
                        })
                        setTree(res);
                    }
                })
        }
    },[SelectedBranch, props.path, window.location.href])

    function ConPath(path: string): string {
        if (path.startsWith("/")) {
                path = path.substring(1);
        }
        if (path === "") {
            return path;
        }
        if (!path.endsWith("/")) {
            path = path + "/";
        }
        return path;
    }

    const breadcrumbs = props.path?.split("/").filter((x)=>x !== "") || [];
    const nav = useNavigate();
    return (
       <>
           {
               context && (
                   <div>
                       <div className="flex items-center gap-1 mb-4 text-sm">
                           <NavLink to={"/" + context.owner.username + "/" + context.model.repo_name} className=" hover:underline">
                               {context.model.repo_name}
                           </NavLink>
                           {breadcrumbs.map((crumb, index) => (
                               <div key={index} className="flex items-center gap-1" onClick={()=>{
                                   const links = breadcrumbs.slice(0, index + 1).join("/")
                                   nav("/" + context.owner.username + "/" + context.model.repo_name + "/tree/" + links);
                               }}>
                                   <ChevronRight className="h-3 w-3 text-muted-foreground" />
                                   <span
                                       className={
                                           index === breadcrumbs.length - 1 ? "font-medium" : "hover:underline cursor-pointer"
                                       }
                                   >
                            {crumb}
                        </span>
                               </div>
                           ))}
                       </div>
                       <Card>
                           <CardHeader>
                               <div className="flex items-center justify-between">
                                   <div className="flex items-center gap-2">
                                       <GitBranch className="h-4 w-4" />
                                       <Select
                                           defaultValue={context?.model.default_head || ""}
                                       >
                                           <SelectTrigger className="w-[180px]">
                                               <SelectValue placeholder="branches" />
                                           </SelectTrigger>
                                           <SelectContent>
                                               {
                                                   Branches.map((branch, index) => (
                                                       <SelectItem key={index} value={branch.branch.ref_name}>{branch.branch.ref_name}</SelectItem>
                                                   ))
                                               }
                                           </SelectContent>
                                       </Select>
                                   </div>
                                   {
                                       breadcrumbs.length > 0 && (
                                           <div className="flex items-center justify-between">
                                               <Button onClick={()=>{
                                                   nav("/" + context.owner.username + "/" + context.model.repo_name + "/tree/" + breadcrumbs.slice(0, breadcrumbs.length - 1).join("/"));
                                               }}>
                                                   <span className="text-sm">Parent directory</span>
                                               </Button>
                                           </div>
                                       )
                                   }
                               </div>

                           </CardHeader>

                           <CardContent className="p-0">

                               <div className="divide-y">
                                   {Tree.map((file, index) => (
                                       <div key={index} className="flex items-center justify-between p-3 hover:bg-muted/50" onClick={()=>{
                                           if (file.item.kind === "Tree") {
                                               console.log(ConPath(props.path + "/" + file.item.name));
                                               nav("/" + context.owner.username + "/" + context.model.repo_name + "/tree/" + ConPath(file.item.path + file.item.name));
                                           }
                                       }}>
                                           <div className="flex items-center gap-3">
                                               {file.item.kind === "Tree" ? (
                                                   <Folder className="h-4 w-4 text-blue-500" />
                                               ) : (
                                                   <File className="h-4 w-4 text-muted-foreground" />
                                               )}
                                               <span className="font-medium hover:underline cursor-pointer">{file.item.name}</span>
                                           </div>
                                           <div className="flex items-center gap-4 text-sm text-muted-foreground">
                                               <span className="hidden md:block">{file.commit_message}</span>
                                               <span>{formatRelativeTime(file.commit_time * 1000)}</span>
                                           </div>
                                       </div>
                                   ))}
                               </div>
                           </CardContent>
                       </Card>
                   </div>
               )
           }
       </>
    )
}
