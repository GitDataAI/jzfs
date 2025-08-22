import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { GitFork, Star } from "lucide-react"
import {useContext, useEffect, useState} from "react";
import {UserDataContext, type UserReposResponse, useUserData} from "@/hooks/use-user-data.tsx";
import {useSearchParams} from "react-router-dom";
import {formatRelativeTime} from "@/lib/utils.ts";
import Pagination from 'rsuite/Pagination';
import 'rsuite/Pagination/styles/index.css';

export default function RepositoriesPage() {
    const context = useContext(UserDataContext);
    const data = useUserData();
    const [SearchParam, setSearchParam] = useSearchParams();
    const page = parseInt(SearchParam.get("page") as string) || 0;
    const limit = parseInt(SearchParam.get("limit") as string) || 50;
    const [ResultData, setResultData] = useState<UserReposResponse | null>();

    useEffect(() => {
        if (context) {
            const FetchData = async () => {
                const res = await data.fetchUserRepos(context.model.username, {
                    page: page,
                    page_size: limit
                });
                setResultData(res);
            }
            FetchData();
        }
    }, [page, limit]);
    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h1 className="text-2xl font-bold">Repositories ({ResultData?.total || 0})</h1>
                <Button>New Repository</Button>
            </div>
            <div className="space-y-4">
                {(ResultData?.items || []).map((value) => (
                    <Card key={value.repo.repo_name}>
                        <CardContent className="pt-6">
                            <div className="space-y-4">
                                <div className="space-y-2">
                                    <h3 className="text-lg font-semibold">{value.repo.repo_name}</h3>
                                    <p className="text-muted-foreground">{value.repo.description}</p>
                                    <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
                                        <span className="flex items-center gap-1">
                                            <Star className="w-4 h-4" />
                                            {value.state.stars}
                                        </span>
                                        <span className="flex items-center gap-1">
                                            <GitFork className="w-4 h-4" />
                                            {value.state.forks}
                                        </span>
                                        <span>Updated {formatRelativeTime((new Date(value.repo.updated_at)).getTime() * 1000 )}</span>
                                    </div>
                                </div>
                                <Button variant="outline" size="sm">
                                    <Star className="w-4 h-4 mr-1" />
                                    Star
                                </Button>
                            </div>
                        </CardContent>
                    </Card>
                ))}
                <Pagination
                    className="flex justify-center content-center"
                    total={ResultData?.total || 0} limit={limit} activePage={page} onChangePage={()=>{
                    setSearchParam({
                        page: (page).toString(),
                        limit: limit.toString()
                    })
                }} />
            </div>
        </div>
    )
}
