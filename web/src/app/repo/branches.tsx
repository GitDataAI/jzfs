"use client"

import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { GitBranch } from "lucide-react"
import {useContext, useEffect, useState} from "react";
import {type BranchInfo, RepoDataContext, useRepoData} from "@/hooks/use-repo-data.tsx";

export function BranchesPage() {
    const context = useContext(RepoDataContext);
    const data = useRepoData();
    const [Branches, setBranches] = useState<BranchInfo[]>([]);
    useEffect(() => {
        if (context) {
            data.fetchRepoBranches(context.owner.username, context.model.repo_name)
                .then(res=>{
                    if (res) {
                        setBranches(res);
                    }
                })
        }
    }, []);
    const isDefault = (branch: BranchInfo) => branch.branch.ref_name === context?.model.default_head;
    return (
        <div>
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-2xl font-bold">Branches</h1>
                <Button>
                    <GitBranch className="h-4 w-4 mr-1" />
                    New branch
                </Button>
            </div>

            <Card>
                <CardContent className="p-0">
                    <div className="divide-y">
                        {Branches.map((branch, index) => (
                            <div key={index} className="flex items-center justify-between p-4">
                                <div className="flex items-center gap-3">
                                    <GitBranch className="h-4 w-4" />
                                    <div>
                                        <div className="flex items-center gap-2">
                                            <span className="font-medium   hover:underline cursor-pointer">{branch.branch.ref_name}</span>
                                            {isDefault(branch) && <Badge variant="outline">default</Badge>}
                                        </div>
                                        <span className="text-sm text-muted-foreground">Updated {branch.branch.updated_at}</span>
                                    </div>
                                </div>
                                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                                    <span className="">head: <span>{branch.branch.ref_git_id}</span></span>
                                    {!isDefault(branch) && (
                                        <Button variant="outline" size="sm">
                                            Delete
                                        </Button>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
