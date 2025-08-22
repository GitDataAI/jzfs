import {useSearchParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator
} from "@/components/ui/breadcrumb.tsx";
import useRepos, {type ReposRecommendResponse} from "@/hooks/use-repos.tsx";
import {toast} from "sonner";
import {RepositoryList} from "@/components/home/repo-list.tsx";
import {Button} from "@/components/ui/button.tsx";

export const RepositoriesLayout = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    let page = parseInt(searchParams.get("page") as string) || 0;
    let limit = parseInt(searchParams.get("limit") as string) || 50;
    const repos = useRepos();
    const [repoListResult, setRepoListResult] = useState<ReposRecommendResponse | null>();
    useEffect(() => {
        const FetchData = async () => {
            try {
                const data = await repos.fetchRecommendedRepos({
                    page: page,
                    page_size: limit
                })
                setRepoListResult(data)
            }catch (e) {
                toast.error(String(e))
            }
        }
        FetchData().then().catch().finally()
    }, [page,limit]);
    const NextPage = () => {
        if (repoListResult && repoListResult.total > repoListResult.page_size * (repoListResult.page + 1)) {
            setSearchParams({
                page: (repoListResult.page + 1).toString(),
                limit: repoListResult.page_size.toString()
            })
        }
    }
    const PrevPage = () => {
        if (repoListResult && repoListResult.page > 1) {
            setSearchParams({
                page: (repoListResult.page - 1).toString(),
                limit: repoListResult.page_size.toString()
            })
        }
    }
    return(
        <>
            <header className="flex h-16 shrink-0 items-center gap-2 px-4 mt-1">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem className="hidden md:block">
                            <BreadcrumbLink href="#">
                                Home
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator className="hidden md:block" />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Repositories</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>
            <div className="bg-background">
                <div className="container mx-auto px-4 py-8">
                    <div className="mb-8">
                        <h1 className="text-3xl font-bold text-foreground mb-2">Repository Dashboard</h1>
                        <p className="text-muted-foreground">Manage and explore your repositories</p>
                    </div>
                    {
                        repoListResult && (
                            <RepositoryList data={repoListResult} />
                        )
                    }
                    <div className="flex justify-center content-center gap-6 mt-2">
                        {
                            page > 1 && (
                                <Button onClick={PrevPage}>Previous</Button>
                            )
                        }
                        {
                            repoListResult && repoListResult.total > limit&& (
                                <Button variant="ghost">{page + 1}</Button>
                             )
                        }
                        {
                            repoListResult && repoListResult.total > repoListResult.page_size * (repoListResult.page + 1) && (
                                <Button onClick={NextPage}>Next</Button>
                            )
                        }
                    </div>
                </div>
            </div>
        </>
    )
}