import { Card, CardContent } from "@/components/ui/card"
import { Star } from "lucide-react"

export default function StarsPage() {
    const starredRepos:{
        name: string,
        description: string,
        language: string,
        stars: number,
        owner: string,
    }[] = []

    return (
        <div className="space-y-6">
            <h1 className="text-2xl font-bold">Starred Repositories ({starredRepos.length})</h1>

            <div className="space-y-4">
                {starredRepos.map((repo) => (
                    <Card key={repo.name}>
                        <CardContent className="pt-6">
                            <div className="space-y-2">
                                <h3 className="text-lg font-semibold text-accent">{repo.name}</h3>
                                <p className="text-muted-foreground">{repo.description}</p>
                                <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
                                    <span>by {repo.owner}</span>
                                    <span className="flex items-center gap-1">
                    <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                                        {repo.language}
                  </span>
                                    <span className="flex items-center gap-1">
                    <Star className="w-4 h-4" />
                                        {repo.stars.toLocaleString()}
                  </span>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                ))}
            </div>
        </div>
    )
}
