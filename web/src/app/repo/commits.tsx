"use client"

import {Card, CardContent, CardFooter, CardHeader} from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import {Avatar, AvatarFallback, AvatarImage} from "@/components/ui/avatar"
import {ChevronLeft, ChevronRight, GitBranch, Hash} from "lucide-react"
import {useContext, useEffect, useState} from "react";
import {type BranchInfo, type CommitInfo, RepoDataContext, useRepoData} from "@/hooks/use-repo-data.tsx";
import {formatRelativeTime} from "@/lib/utils.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

export function CommitsPage() {
    const context = useContext(RepoDataContext);
    const data = useRepoData();
    const [Pager, setPager] = useState({
        page: 1,
        page_size: 10
    })
    const [Branches, setBranches] = useState<BranchInfo[]>([]);
    const [SelectBranch, setSelectBranch] = useState<BranchInfo | undefined>()
    const [Commits, setCommits] = useState<CommitInfo[]>([]);
    const [Total, setTotal] = useState(0);
    
    // 分页处理函数
    const goToPrevious = () => {
        if (Pager.page > 1) {
            setPager(prev => ({ ...prev, page: prev.page - 1 }));
        }
    };
    
    const goToNext = () => {
        const totalPages = Math.ceil(Total / Pager.page_size);
        if (Pager.page < totalPages) {
            setPager(prev => ({ ...prev, page: prev.page + 1 }));
        }
    };
    
    const goToPage = (page: number) => {
        setPager(prev => ({ ...prev, page }));
    };
    
    useEffect(() => {
        if (SelectBranch && context) {
            data.fetchRepoCommits(context.owner.username, context.model.repo_name, SelectBranch.branch.ref_name, Pager.page, Pager.page_size)
                .then(res=>{
                    if (res) {
                        setCommits(res.data);
                        setTotal(res.total);
                    }
                })
        }
    }, [Pager.page]);
    useEffect(() => {
        if (context) {
            if (context) {
                data.fetchRepoBranches(context.owner.username, context.model.repo_name)
                    .then(res=>{
                        if (res) {
                            setBranches(res);
                            const defaultBranch = res.find(branch=> branch.branch.ref_name===context.model.default_head);
                            setSelectBranch(defaultBranch);
                            console.log(defaultBranch);
                            if (defaultBranch) {
                                data.fetchRepoCommits(context.owner.username, context.model.repo_name, defaultBranch.branch.ref_name, Pager.page, Pager.page_size)
                                    .then(res=>{
                                        if (res) {
                                            setCommits(res.data);
                                            setTotal(res.total);
                                        }
                                    })
                            }
                        }
                    })
            }
        }
    }, []);
    useEffect(() => {
        if (SelectBranch && context) {
            data.fetchRepoCommits(context.owner.username, context.model.repo_name, SelectBranch.branch.ref_name, Pager.page, Pager.page_size)
                .then(res=>{
                    if (res) {
                        setCommits(res.data);
                        setTotal(res.total);
                    }
                })
        }
    }, [SelectBranch]);
    return (
        <div>
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-2xl font-bold">Commits</h1>
                <span>
                    {Total} commits
                </span>
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
                    </div>
                </CardHeader>
                <CardContent className="p-0">
                    <div className="divide-y">
                        {Commits.map((commit, index) => (
                            <div key={index} className="flex items-center justify-between p-4 hover:bg-muted/50">
                                <div className="flex items-center gap-3">
                                    <Avatar className="h-8 w-8">
                                        <AvatarImage src={ commit.author.avatar || "" } alt={ commit.author.name } />
                                        <AvatarFallback>
                                            {commit.author.name }
                                        </AvatarFallback>
                                    </Avatar>
                                    <div>
                                        <div className="font-medium   hover:underline cursor-pointer">{commit.message}</div>
                                        <div className="text-sm text-muted-foreground">
                                            {commit.author.name } committed {formatRelativeTime(commit.time * 1000)}
                                            {commit.commiter.email === "no-replay@gitdata.ai" && <span className="ml-2 text-green-600">✓ Verified</span>}
                                        </div>
                                    </div>
                                </div>
                                <div className="flex items-center gap-2">
                                    <code className="text-sm bg-muted px-2 py-1 rounded font-mono">{commit.commit_id}</code>
                                    <Button variant="outline" size="sm">
                                        <Hash className="h-4 w-4" />
                                    </Button>
                                </div>
                            </div>
                        ))}
                    </div>
                </CardContent>
                <CardFooter>
                    <div className="flex items-center justify-center w-full">
                        {Total > 0 && (
                            <div className="flex items-center justify-center gap-2">
                                <Button variant="outline" size="sm" onClick={goToPrevious} disabled={Pager.page === 1}>
                                    <ChevronLeft className="h-4 w-4" />
                                    Previous
                                </Button>
                                
                                <div className="flex items-center gap-1">
                                    {Pager.page > 3 && (
                                        <>
                                            <Button variant={1 === Pager.page ? "default" : "outline"} size="sm" onClick={() => goToPage(1)}>
                                                1
                                            </Button>
                                            {Pager.page > 4 && <span className="px-2">...</span>}
                                        </>
                                    )}
                                    {Array.from({ length: Math.min(5, Math.ceil(Total / Pager.page_size)) }, (_, i) => {
                                        const pageNum = Math.max(1, Math.min(Pager.page - 2 + i, Math.ceil(Total / Pager.page_size)));
                                        if (pageNum < 1 || pageNum > Math.ceil(Total / Pager.page_size)) return null;
                                        
                                        if (Pager.page <= 3) {
                                            const page = i + 1;
                                            if (page > Math.ceil(Total / Pager.page_size)) return null;
                                            return (
                                                <Button
                                                    key={page}
                                                    variant={page === Pager.page ? "default" : "outline"}
                                                    size="sm"
                                                    onClick={() => goToPage(page)}
                                                >
                                                    {page}
                                                </Button>
                                            );
                                        } else if (Pager.page >= Math.ceil(Total / Pager.page_size) - 2) {
                                            const page = Math.ceil(Total / Pager.page_size) - 4 + i;
                                            if (page < 1 || page > Math.ceil(Total / Pager.page_size)) return null;
                                            return (
                                                <Button
                                                    key={page}
                                                    variant={page === Pager.page ? "default" : "outline"}
                                                    size="sm"
                                                    onClick={() => goToPage(page)}
                                                >
                                                    {page}
                                                </Button>
                                            );
                                        } else {
                                            return (
                                                <Button
                                                    key={pageNum}
                                                    variant={pageNum === Pager.page ? "default" : "outline"}
                                                    size="sm"
                                                    onClick={() => goToPage(pageNum)}
                                                >
                                                    {pageNum}
                                                </Button>
                                            );
                                        }
                                    })}
                                    {Pager.page < Math.ceil(Total / Pager.page_size) - 2 && (
                                        <>
                                            {Pager.page < Math.ceil(Total / Pager.page_size) - 3 && <span className="px-2">...</span>}
                                            <Button
                                                variant={Math.ceil(Total / Pager.page_size) === Pager.page ? "default" : "outline"}
                                                size="sm"
                                                onClick={() => goToPage(Math.ceil(Total / Pager.page_size))}
                                            >
                                                {Math.ceil(Total / Pager.page_size)}
                                            </Button>
                                        </>
                                    )}
                                </div>
                                
                                <Button variant="outline" size="sm" onClick={goToNext} disabled={Pager.page === Math.ceil(Total / Pager.page_size)}>
                                    Next
                                    <ChevronRight className="h-4 w-4" />
                                </Button>
                            </div>
                        )}
                    </div>
                </CardFooter>
            </Card>
        </div>
    )
}
