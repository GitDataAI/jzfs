import { Star, GitFork, Eye, Lock, Globe, Calendar } from "lucide-react"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import type {RepoItem} from "@/hooks/use-repos.tsx";
import { formatDistanceToNow } from "date-fns"
import {useNavigate} from "react-router-dom";
interface RepositoryCardProps {
    item: RepoItem
    viewMode: "grid" | "list"
}

export function RepositoryCard({ item, viewMode }: RepositoryCardProps) {
    const { owner, repo, state } = item
    const nav = useNavigate();
    if (viewMode === "list") {
        return (
            <Card className="hover:shadow-md transition-shadow" onClick={()=>{
                nav("/" + owner.username + "/" + repo.repo_name)
            }}>
                <CardContent className="p-6">
                    <div className="flex items-start justify-between">
                        <div className="flex items-start space-x-4 flex-1">
                            <Avatar className="h-10 w-10">
                                <AvatarImage src={owner.avatar_url || undefined} />
                                <AvatarFallback>{owner.username.charAt(0).toUpperCase()}</AvatarFallback>
                            </Avatar>
                            <div className="flex-1 min-w-0">
                                <div className="flex items-center space-x-2 mb-2">
                                    <h3 className="text-lg font-semibold text-foreground truncate">
                                        {repo.namespace}/{repo.repo_name}
                                    </h3>
                                    <Badge variant={repo.is_private ? "secondary" : "outline"}>
                                        {repo.is_private ? (
                                            <>
                                                <Lock className="h-3 w-3 mr-1" />
                                                Private
                                            </>
                                        ) : (
                                            <>
                                                <Globe className="h-3 w-3 mr-1" />
                                                Public
                                            </>
                                        )}
                                    </Badge>
                                </div>
                                <p className="text-muted-foreground text-sm mb-3 line-clamp-2">{repo.description}</p>
                                <div className="flex items-center space-x-4 text-sm text-muted-foreground">
                                    <div className="flex items-center space-x-1">
                                        <Star className="h-4 w-4" />
                                        <span>{state.stars}</span>
                                    </div>
                                    <div className="flex items-center space-x-1">
                                        <GitFork className="h-4 w-4" />
                                        <span>{state.forks}</span>
                                    </div>
                                    <div className="flex items-center space-x-1">
                                        <Eye className="h-4 w-4" />
                                        <span>{state.watches}</span>
                                    </div>
                                    <div className="flex items-center space-x-1">
                                        <Calendar className="h-4 w-4" />
                                        <span>Updated {formatDistanceToNow(new Date(repo.updated_at), { addSuffix: true })}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </CardContent>
            </Card>
        )
    }

    return (
        <Card className="hover:shadow-md transition-shadow h-full" onClick={()=>{
            nav("/" + owner.username + "/" + repo.repo_name)
        }}>
            <CardHeader className="pb-3">
                <div className="flex items-start justify-between">
                    <div className="flex items-center space-x-3">
                        <Avatar className="h-8 w-8">
                            <AvatarImage src={owner.avatar_url || undefined} />
                            <AvatarFallback>{owner.username.charAt(0).toUpperCase()}</AvatarFallback>
                        </Avatar>
                        <div className="min-w-0 flex-1">
                            <h3 className="font-semibold text-foreground truncate">{repo.repo_name}</h3>
                            <p className="text-sm text-muted-foreground">{owner.username}</p>
                        </div>
                    </div>
                    <Badge variant={repo.is_private ? "secondary" : "outline"} className="ml-2">
                        {repo.is_private ? (
                            <>
                                <Lock className="h-3 w-3 mr-1" />
                                Private
                            </>
                        ) : (
                            <>
                                <Globe className="h-3 w-3 mr-1" />
                                Public
                            </>
                        )}
                    </Badge>
                </div>
            </CardHeader>
            <CardContent className="pt-0">
                <p className="text-sm text-muted-foreground mb-4 line-clamp-3">{repo.description}</p>
                <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <div className="flex items-center space-x-3">
                        <div className="flex items-center space-x-1">
                            <Star className="h-4 w-4" />
                            <span>{state.stars}</span>
                        </div>
                        <div className="flex items-center space-x-1">
                            <GitFork className="h-4 w-4" />
                            <span>{state.forks}</span>
                        </div>
                        <div className="flex items-center space-x-1">
                            <Eye className="h-4 w-4" />
                            <span>{state.watches}</span>
                        </div>
                    </div>
                </div>
                <div className="mt-3 pt-3 border-t border-border">
                    <div className="flex items-center space-x-1 text-xs text-muted-foreground">
                        <Calendar className="h-3 w-3" />
                        <span>Updated {formatDistanceToNow(new Date(repo.updated_at), { addSuffix: true })}</span>
                    </div>
                </div>
            </CardContent>
        </Card>
    )
}
