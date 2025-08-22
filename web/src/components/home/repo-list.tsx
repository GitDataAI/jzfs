
import { useState, useMemo } from "react"
import { Search, Filter, Grid, List } from "lucide-react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import type {ReposRecommendResponse} from "@/hooks/use-repos.tsx";
import {RepositoryCard} from "@/components/home/repo-card.tsx";
import {useNavigate} from "react-router-dom";

interface RepositoryListProps {
    data: ReposRecommendResponse
}

export function RepositoryList({ data }: RepositoryListProps) {
    const [searchQuery, setSearchQuery] = useState("")
    const [sortBy, setSortBy] = useState("updated_at")
    const [filterBy, setFilterBy] = useState("all")
    const [viewMode, setViewMode] = useState<"grid" | "list">("grid")
    const nav = useNavigate();
    const filteredAndSortedItems = useMemo(() => {
        const filtered = data.items.filter((item) => {
            const matchesSearch =
                item.repo.repo_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                (item.repo.description || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
                item.owner.username.toLowerCase().includes(searchQuery.toLowerCase())

            const matchesFilter =
                filterBy === "all" ||
                (filterBy === "private" && item.repo.is_private) ||
                (filterBy === "public" && !item.repo.is_private)
            return matchesSearch && matchesFilter
        })

        return filtered.sort((a, b) => {
            switch (sortBy) {
                case "name":
                    return a.repo.repo_name.localeCompare(b.repo.repo_name)
                case "stars":
                    return b.state.stars - a.state.stars
                case "forks":
                    return b.state.forks - a.state.forks
                case "updated_at":
                default:
                    return new Date(b.repo.updated_at).getTime() - new Date(a.repo.updated_at).getTime()
            }
        })
    }, [data.items, searchQuery, sortBy, filterBy])

    return (
        <div className="space-y-6">
            <div className="flex flex-col sm:flex-row gap-4 items-center justify-between">
                <div className="flex flex-1 items-center space-x-2">
                    <div className="relative flex-1 max-w-sm">
                        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
                        <Input
                            placeholder="Search repositories..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="pl-10"
                        />
                    </div>
                    <Select value={filterBy} onValueChange={setFilterBy}>
                        <SelectTrigger className="w-32">
                            <Filter className="h-4 w-4 mr-2" />
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="all">All</SelectItem>
                            <SelectItem value="public">Public</SelectItem>
                            <SelectItem value="private">Private</SelectItem>
                        </SelectContent>
                    </Select>
                    <Select value={sortBy} onValueChange={setSortBy}>
                        <SelectTrigger className="w-40">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="updated_at">Last Updated</SelectItem>
                            <SelectItem value="name">Name</SelectItem>
                            <SelectItem value="stars">Stars</SelectItem>
                            <SelectItem value="forks">Forks</SelectItem>
                        </SelectContent>
                    </Select>
                    <Button onClick={() => {
                        nav("/init/repository")
                    }}>
                        Initialize Repository
                    </Button>
                </div>
                <div className="flex items-center space-x-2">
                    <Button variant={viewMode === "grid" ? "default" : "outline"} size="sm" onClick={() => setViewMode("grid")}>
                        <Grid className="h-4 w-4" />
                    </Button>
                    <Button variant={viewMode === "list" ? "default" : "outline"} size="sm" onClick={() => setViewMode("list")}>
                        <List className="h-4 w-4" />
                    </Button>
                </div>
            </div>

            {/* Results Summary */}
            <div className="text-sm text-muted-foreground">
                Showing {filteredAndSortedItems.length} of {data.total} repositories
            </div>

            {/* Repository Grid/List */}
            <div className={`${viewMode === "grid" ? "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6" : "space-y-4"}`}>
                {filteredAndSortedItems.map((item) => (
                    <RepositoryCard key={item.repo.uid} item={item} viewMode={viewMode} />
                ))}
            </div>

            {filteredAndSortedItems.length === 0 && (
                <div className="text-center py-12">
                    <p className="text-muted-foreground">No repositories found matching your criteria.</p>
                </div>
            )}
        </div>
    )
}
