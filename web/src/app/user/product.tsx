import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Package } from "lucide-react"

export default function ProductsPage() {
    const products = [
        {
            name: "Test Data",
            description: "Professional development tools for modern workflows",
            price: "$29",
            users: "10k+",
        },
    ]

    return (
        <div className="space-y-6">
            <h1 className="text-2xl font-bold">Products ({products.length})</h1>

            <div className="space-y-4">
                {products.map((product) => (
                    <Card key={product.name}>
                        <CardHeader>
                            <CardTitle className="flex items-center gap-2">
                                <Package className="w-5 h-5" />
                                {product.name}
                            </CardTitle>
                            <CardDescription>{product.description}</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div className="flex justify-between items-center">
                                <div>
                                    <p className="text-2xl font-bold">{product.price}</p>
                                    <p className="text-sm text-muted-foreground">{product.users} users</p>
                                </div>
                                <Button>Learn More</Button>
                            </div>
                        </CardContent>
                    </Card>
                ))}
            </div>
        </div>
    )
}
