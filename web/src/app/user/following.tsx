import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"

export default function FollowingPage() {
    return (
        <div className="space-y-6">
            <h1 className="text-2xl font-bold">Social</h1>

            <div className="space-y-6">
                <Card>
                    <CardHeader>
                        <CardTitle>Following (567)</CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-4">
                        {Array.from({ length: 1 }, (_, i) => (
                            <div key={i} className="flex items-center justify-between">
                                <div className="flex items-center gap-3">
                                    <Avatar className="w-10 h-10">
                                        <AvatarImage src={`/developer-avatar.png?height=40&width=40&query=developer avatar ${i}`} />
                                        <AvatarFallback>U{i}</AvatarFallback>
                                    </Avatar>
                                    <div>
                                        <p className="font-medium">User {i + 1}</p>
                                        <p className="text-sm text-muted-foreground">@user{i + 1}</p>
                                    </div>
                                </div>
                                <Button variant="outline" size="sm">
                                    Unfollow
                                </Button>
                            </div>
                        ))}
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <CardTitle>Followers (1,234)</CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-4">
                        {Array.from({ length: 1 }, (_, i) => (
                            <div key={i} className="flex items-center justify-between">
                                <div className="flex items-center gap-3">
                                    <Avatar className="w-10 h-10">
                                        <AvatarImage src={`/follower-avatar.png?height=40&width=40&query=follower avatar ${i}`} />
                                        <AvatarFallback>F{i}</AvatarFallback>
                                    </Avatar>
                                    <div>
                                        <p className="font-medium">Follower {i + 1}</p>
                                        <p className="text-sm text-muted-foreground">@follower{i + 1}</p>
                                    </div>
                                </div>
                                <Button size="sm">Follow</Button>
                            </div>
                        ))}
                    </CardContent>
                </Card>
            </div>
        </div>
    )
}
