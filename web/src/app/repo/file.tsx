"use client"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import {
    GitBranch,
    File,
    Folder,
    Download,
    Copy,
    Book,
    Activity,
    Clock,
    Star,
    Eye,
    GitFork,
} from "lucide-react"
import {useContext, useEffect, useState} from "react";
import {type BranchInfo, RepoDataContext, type TreeItemLastCommit, useRepoData} from "@/hooks/use-repo-data.tsx";
import {formatRelativeTime} from "@/lib/utils.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {useNavigate} from "react-router-dom";


export function FilesPage() {
    const context = useContext(RepoDataContext);
    const data = useRepoData();
    const [Branches, setBranches] = useState<BranchInfo[]>([]);
    const [SelectedBranch, setSelectedBranch] = useState<BranchInfo | null>(null);
    const [Tree, setTree] = useState<TreeItemLastCommit[]>([]);
    const [total_commit, set_total_commit] = useState(0);
    useEffect(() => {
        if (context) {
            data.fetchRepoBranches(context.owner.username, context.model.repo_name)
                .then(res=>{
                    if (res) {
                        setBranches(res);
                        const page_branch = res.find((b)=>b.branch.ref_name === context.model.default_head) || res[0];
                        setSelectedBranch(page_branch);
                        data.fetchRepoCommits(context.owner.username, context.model.repo_name, page_branch.branch.ref_name, 0, 1)
                            .then(res=>{
                                if (res) {
                                    set_total_commit(res.total);
                                }
                            })
                        data.fetchRepoTree(context.owner.username, context.model.repo_name, page_branch.branch.ref_name)
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
    useEffect(() => {
        if (SelectedBranch && context ) {
            data.fetchRepoTree(context.owner.username, context.model.repo_name, SelectedBranch.branch.ref_name)
                .then(res=>{
                    if (res) {
                        res.sort((a, b)=>{
                            if (a.item.kind === "Tree" && b.item.kind !== "Tree") {
                                return -1;
                            } else if (a.item.kind !== "Tree" && b.item.kind === "Tree") {
                                return 1;
                            }
                            return a.item.name.localeCompare(b.item.name);
                        })
                        setTree(res);
                    }
                })
        }
    }, [SelectedBranch]);
    const nav = useNavigate();
    return (
        <>
            {
                context && Branches && (
                    <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
                        {
                            Branches.length !== 0 ? (
                                <div className="lg:col-span-3">
                                    <div className="flex items-center justify-between mb-4">
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
                                        <div className="flex items-center gap-2">
                                            <span className="text-sm text-muted-foreground">{total_commit} commits</span>
                                        </div>
                                    </div>

                                    <Card>
                                        <CardContent className="p-0">
                                            <div className="divide-y">
                                                {Tree.map((file, index) => (
                                                    <div key={index} className="flex items-center justify-between p-3 hover:bg-muted/50" onClick={()=>{
                                                        if (file.item.kind === "Tree") {
                                                            nav(`/${context.owner.username}/${context.model.repo_name}/tree/${file.item.name}`)
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
                                    <Card className="mt-6">
                                        <CardHeader className="pb-3">
                                            <div className="flex items-center gap-2">
                                                <Book className="h-4 w-4" />
                                                <span className="font-medium">README.md</span>
                                            </div>
                                        </CardHeader>
                                        <CardContent>
                                            {/*{TODO README FILE}*/}
                                        </CardContent>
                                    </Card>
                                </div>
                            ):(
                                <div className="lg:col-span-3 flex items-center justify-center h-full">
                                    <span className="text-muted-foreground">No branches found</span>
                                </div>
                            )
                        }
                        <div className="lg:col-span-1">
                            <div className="space-y-4">
                                <Card>
                                    <CardHeader className="pb-3">
                                        <div className="flex items-center justify-between">
                                            <span className="font-medium text-sm">Clone</span>
                                            <Button variant="outline" size="sm">
                                                <Download className="h-4 w-4 mr-1" />
                                                Code
                                            </Button>
                                        </div>
                                    </CardHeader>
                                    <CardContent className="space-y-3">
                                        <div>
                                            <div className="flex items-center justify-between mb-1">
                                                <span className="text-xs font-medium text-muted-foreground">HTTPS</span>
                                                <Button variant="ghost" size="sm" className="h-6 w-6 p-0">
                                                    <Copy className="h-3 w-3" />
                                                </Button>
                                            </div>
                                            <div className="bg-muted p-2 rounded text-xs font-mono break-all">
                                                {
                                                    // @ts-ignore
                                                    window.location.git_http_url + context.owner.username + "/" + context.model.repo_name}
                                            </div>
                                        </div>
                                        <div>
                                            <div className="flex items-center justify-between mb-1">
                                                <span className="text-xs font-medium text-muted-foreground">SSH</span>
                                                <Button variant="ghost" size="sm" className="h-6 w-6 p-0">
                                                    <Copy className="h-3 w-3" />
                                                </Button>
                                            </div>
                                            <div className="bg-muted p-2 rounded text-xs font-mono break-all">
                                                {
                                                    // @ts-ignore
                                                    window.location.git_ssh_url  + context.owner.username + "/" + context.model.repo_name}
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                                <Card>
                                    <CardHeader className="pb-3">
                                        <span className="font-medium text-sm">About</span>
                                    </CardHeader>
                                    <CardContent className="space-y-3">
                                        <p className="text-sm text-muted-foreground">
                                            {context.model.description || ""}
                                        </p>
                                        <div className="space-y-2">
                                            <div className="flex items-center gap-2 text-sm">
                                                <Star className="h-3 w-3" />
                                                <span>0 stars</span>
                                            </div>
                                            <div className="flex items-center gap-2 text-sm">
                                                <Eye className="h-3 w-3" />
                                                <span>0 watching</span>
                                            </div>
                                            <div className="flex items-center gap-2 text-sm">
                                                <GitFork className="h-3 w-3" />
                                                <span>0 forks</span>
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                                {/* Products section */}
                                <Card>
                                    <CardHeader className="pb-3">
                                        <div className="flex items-center justify-between">
                                            <span className="font-medium text-sm">Products</span>
                                            <Badge variant="secondary">0</Badge>
                                        </div>
                                    </CardHeader>
                                    <CardContent>
                                        <div className="space-y-2">
                                        {/*    <div className="flex items-center gap-2">*/}
                                        {/*        <Tag className="h-3 w-3 text-green-600" />*/}
                                        {/*        <span className="text-sm font-medium">v2.1.0</span>*/}
                                        {/*    </div>*/}
                                        {/*    <p className="text-xs text-muted-foreground">Latest product</p>*/}
                                        {/*    <Button variant="outline" size="sm" className="w-full bg-transparent">*/}
                                        {/*        View all products*/}
                                        {/*    </Button>*/}

                                            <div className="flex items-center gap-2">
                                                <span className="text-sm font-medium">
                                                    No products
                                                </span>
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                                <Card>
                                    <CardHeader className="pb-3">
                                        <span className="font-medium text-sm">Activity</span>
                                    </CardHeader>
                                    <CardContent>
                                        <div className="space-y-2">
                                            <div className="flex items-center gap-2 text-xs">
                                                <Activity className="h-3 w-3" />
                                                <span>Active in the last week</span>
                                            </div>
                                            <div className="flex items-center gap-2 text-xs">
                                                <Clock className="h-3 w-3" />
                                                <span>
                                                    Last updated {formatRelativeTime((new Date(context.model.updated_at)).getTime())}
                                                </span>
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                            </div>
                        </div>
                    </div>
                )
            }
        </>
    )
}
