import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Check, X } from "lucide-react"
import { NavLink } from "react-router-dom";

export default function PricingPage() {
    const plans = [
        {
            name: "Starter",
            price: "Free",
            description: "Perfect for individual developers and small projects",
            features: [
                { name: "Up to 3 datasets", included: true },
                { name: "Basic version control", included: true },
                { name: "5GB storage", included: true },
                { name: "Community support", included: true },
                { name: "Basic data analysis", included: true },
                { name: "Advanced analytics", included: false },
                { name: "Team collaboration", included: false },
                { name: "Priority support", included: false },
                { name: "Custom integrations", included: false },
            ],
            cta: "Get Started Free",
            popular: false,
        },
        {
            name: "Professional",
            price: "$49",
            period: "/month",
            description: "Ideal for growing teams and production workloads",
            features: [
                { name: "Unlimited datasets", included: true },
                { name: "Advanced version control", included: true },
                { name: "100GB storage", included: true },
                { name: "Email support", included: true },
                { name: "Advanced data analysis", included: true },
                { name: "Advanced analytics", included: true },
                { name: "Team collaboration (up to 10)", included: true },
                { name: "Priority support", included: false },
                { name: "Custom integrations", included: false },
            ],
            cta: "Start Free Trial",
            popular: true,
        },
        {
            name: "Enterprise",
            price: "Custom",
            description: "For large organizations with complex requirements",
            features: [
                { name: "Unlimited datasets", included: true },
                { name: "Enterprise version control", included: true },
                { name: "Unlimited storage", included: true },
                { name: "24/7 phone & email support", included: true },
                { name: "Advanced data analysis", included: true },
                { name: "Advanced analytics", included: true },
                { name: "Unlimited team members", included: true },
                { name: "Priority support", included: true },
                { name: "Custom integrations", included: true },
            ],
            cta: "Contact Sales",
            popular: false,
        },
    ]

    return (
        <div className="min-h-screen bg-background">
            <section className="py-20 px-4 sm:px-6 lg:px-8">
                <div className="container mx-auto max-w-4xl text-center">
                    <Badge variant="secondary" className="mb-4">
                        Pricing Plans
                    </Badge>
                    <h1 className="text-4xl sm:text-5xl font-bold text-foreground mb-6">
                        Choose the Right Plan
                        <span className="text-primary block mt-2">for Your ML Journey</span>
                    </h1>
                    <p className="text-xl text-muted-foreground mb-8 max-w-3xl mx-auto">
                        From individual developers to enterprise teams, we have a plan that scales with your machine learning data
                        management needs.
                    </p>
                </div>
            </section>

            {/* Pricing Cards */}
            <section className="py-16 px-4 sm:px-6 lg:px-8">
                <div className="container mx-auto max-w-7xl">
                    <div className="grid lg:grid-cols-3 gap-8">
                        {plans.map((plan, index) => (
                            <Card key={plan.name + index} className={`relative ${plan.popular ? "border-primary shadow-lg scale-105" : ""}`}>
                                {plan.popular && (
                                    <Badge className="absolute -top-3 left-1/2 transform -translate-x-1/2 bg-primary">Most Popular</Badge>
                                )}
                                <CardHeader className="text-center pb-8">
                                    <CardTitle className="text-2xl">{plan.name}</CardTitle>
                                    <div className="mt-4">
                                        <span className="text-4xl font-bold">{plan.price}</span>
                                        {plan.period && <span className="text-muted-foreground">{plan.period}</span>}
                                    </div>
                                    <CardDescription className="mt-4">{plan.description}</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <Button
                                        className={`w-full mb-6 ${plan.popular ? "bg-primary hover:bg-primary/90" : ""}`}
                                        variant={plan.popular ? "default" : "outline"}
                                    >
                                        {plan.cta}
                                    </Button>
                                    <ul className="space-y-3">
                                        {plan.features.map((feature, featureIndex) => (
                                            <li key={featureIndex} className="flex items-center gap-3">
                                                {feature.included ? (
                                                    <Check className="w-5 h-5 text-primary flex-shrink-0" />
                                                ) : (
                                                    <X className="w-5 h-5 text-muted-foreground flex-shrink-0" />
                                                )}
                                                <span className={feature.included ? "text-foreground" : "text-muted-foreground"}>
                          {feature.name}
                        </span>
                                            </li>
                                        ))}
                                    </ul>
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                </div>
            </section>

            {/* Feature Comparison */}
            <section className="py-16 px-4 sm:px-6 lg:px-8 bg-muted/50">
                <div className="container mx-auto max-w-6xl">
                    <div className="text-center mb-12">
                        <h2 className="text-3xl font-bold mb-4">Detailed Feature Comparison</h2>
                        <p className="text-xl text-muted-foreground">Compare all features across our pricing plans</p>
                    </div>

                    <div className="overflow-x-auto">
                        <table className="w-full border-collapse bg-background rounded-lg overflow-hidden shadow-sm">
                            <thead>
                            <tr className="border-b border-border">
                                <th className="text-left p-4 font-semibold">Features</th>
                                <th className="text-center p-4 font-semibold">Starter</th>
                                <th className="text-center p-4 font-semibold bg-primary/5">Professional</th>
                                <th className="text-center p-4 font-semibold">Enterprise</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">Dataset Limit</td>
                                <td className="text-center p-4">3 datasets</td>
                                <td className="text-center p-4 bg-primary/5">Unlimited</td>
                                <td className="text-center p-4">Unlimited</td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">Storage</td>
                                <td className="text-center p-4">5GB</td>
                                <td className="text-center p-4 bg-primary/5">100GB</td>
                                <td className="text-center p-4">Unlimited</td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">Team Members</td>
                                <td className="text-center p-4">1</td>
                                <td className="text-center p-4 bg-primary/5">Up to 10</td>
                                <td className="text-center p-4">Unlimited</td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">JiaoZiFS Access</td>
                                <td className="text-center p-4">
                                    <Check className="w-5 h-5 text-primary mx-auto" />
                                </td>
                                <td className="text-center p-4 bg-primary/5">
                                    <Check className="w-5 h-5 text-primary mx-auto" />
                                </td>
                                <td className="text-center p-4">
                                    <Check className="w-5 h-5 text-primary mx-auto" />
                                </td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">Data Visualization</td>
                                <td className="text-center p-4">Basic</td>
                                <td className="text-center p-4 bg-primary/5">Advanced</td>
                                <td className="text-center p-4">Advanced + Custom</td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">API Access</td>
                                <td className="text-center p-4">Limited</td>
                                <td className="text-center p-4 bg-primary/5">Full</td>
                                <td className="text-center p-4">Full + Custom</td>
                            </tr>
                            <tr className="border-b border-border">
                                <td className="p-4 font-medium">Support</td>
                                <td className="text-center p-4">Community</td>
                                <td className="text-center p-4 bg-primary/5">Email</td>
                                <td className="text-center p-4">24/7 Phone & Email</td>
                            </tr>
                            <tr>
                                <td className="p-4 font-medium">SLA</td>
                                <td className="text-center p-4">-</td>
                                <td className="text-center p-4 bg-primary/5">99.9%</td>
                                <td className="text-center p-4">99.99%</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </section>

            {/* FAQ Section */}
            <section className="py-16 px-4 sm:px-6 lg:px-8">
                <div className="container mx-auto max-w-4xl">
                    <div className="text-center mb-12">
                        <h2 className="text-3xl font-bold mb-4">Frequently Asked Questions</h2>
                        <p className="text-xl text-muted-foreground">
                            Get answers to common questions about our pricing and features
                        </p>
                    </div>

                    <div className="grid gap-6">
                        <Card>
                            <CardHeader>
                                <CardTitle className="text-lg">Can I upgrade or downgrade my plan anytime?</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Yes, you can upgrade or downgrade your plan at any time. Changes take effect immediately, and we'll
                                    prorate any billing adjustments.
                                </p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle className="text-lg">Is there a free trial for paid plans?</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Yes, we offer a 14-day free trial for our Professional plan. No credit card required to start your
                                    trial.
                                </p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle className="text-lg">What happens to my data if I cancel?</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Your data remains accessible for 30 days after cancellation. You can export all your datasets and
                                    version history during this period.
                                </p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle className="text-lg">Do you offer custom enterprise solutions?</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Yes, our Enterprise plan can be customized to meet specific requirements including on-premise
                                    deployment, custom integrations, and dedicated support.
                                </p>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </section>

            {/* CTA Section */}
            <section className="py-16 px-4 sm:px-6 lg:px-8 bg-primary/5">
                <div className="container mx-auto max-w-4xl text-center">
                    <h2 className="text-3xl font-bold mb-4">Ready to Get Started?</h2>
                    <p className="text-xl text-muted-foreground mb-8">
                        Join thousands of ML teams already using GitData.ai to manage their datasets
                    </p>
                    <div className="flex flex-col sm:flex-row gap-4 justify-center">
                        <Button size="lg" asChild>
                            <NavLink to="/signup">Start Free Trial</NavLink>
                        </Button>
                        <Button size="lg" variant="outline" asChild>
                            <NavLink to="/contact">Contact Sales</NavLink>
                        </Button>
                    </div>
                </div>
            </section>
        </div>
    )
}
