"use client"

import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Tag, Download } from "lucide-react"

export function TagsPage() {
    const Tags:{
        name: string,
        message: string,
        time: string,
        isPrerelease: boolean
    }[] = [
    ]

    return (
        <div>
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-2xl font-bold">Tags</h1>
                <Button>
                    <Tag className="h-4 w-4 mr-1" />
                    Create tag
                </Button>
            </div>
            <Card>
                <CardContent className="p-0">
                    <div className="divide-y">
                        {Tags.map((tag, index) => (
                            <div key={index} className="flex items-center justify-between p-4 hover:bg-muted/50">
                                <div className="flex items-center gap-3">
                                    <Tag className="h-4 w-4 text-green-600" />
                                    <div>
                                        <div className="flex items-center gap-2">
                                            <span className="font-medium   hover:underline cursor-pointer">{tag.name}</span>
                                            {tag.isPrerelease && <Badge variant="outline">Pre-release</Badge>}
                                            {index === 0 && <Badge variant="secondary">Latest</Badge>}
                                        </div>
                                        <div className="text-sm text-muted-foreground">{tag.message}</div>
                                    </div>
                                </div>
                                <div className="flex items-center gap-2">
                                    <span className="text-sm text-muted-foreground">{tag.time}</span>
                                    <Button variant="outline" size="sm">
                                        <Download className="h-4 w-4" />
                                    </Button>
                                </div>
                            </div>
                        ))}
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
