import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"

export default function OverviewPage() {
    return (
        <div className="space-y-6">
            <div className="grid gap-6">
                <Card>
                    <CardHeader>
                        <CardTitle>Contribution Graph</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="grid grid-cols-7 gap-1">
                            {Array.from({ length: 49 }, (_, i) => (
                                <div
                                    key={i}
                                    className={`w-3 h-3 rounded-sm ${
                                        Math.random() > 0.7
                                            ? "bg-accent"
                                            : Math.random() > 0.5
                                                ? "bg-accent/60"
                                                : Math.random() > 0.3
                                                    ? "bg-accent/30"
                                                    : "bg-muted"
                                    }`}
                                />
                            ))}
                        </div>
                        <p className="text-xs text-muted-foreground mt-2">245 contributions in the last year</p>
                    </CardContent>
                </Card>
            </div>
        </div>
    )
}
