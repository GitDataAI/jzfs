import {Card, CardContent, CardDescription, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {NavLink, useNavigate} from "react-router-dom";
import {useState} from "react";

export const AboutHome = () => {
    const nav = useNavigate();
    const [Email, setEmail] = useState("");
    return(
        <>
            <section className="py-20 px-4 sm:px-6 lg:px-8">
                <div className="container mx-auto max-w-4xl text-center">
                    <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold text-foreground mb-6">
                        Git for Machine Learning
                        <span className="text-primary block mt-2">Data Management</span>
                    </h1>
                    <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
                        Apply Git concepts to ML dataset management. Build the foundation and storage engine for machine learning
                        platforms to better serve model training, evaluation, and inference.
                    </p>
                    <Card className="max-w-md mx-auto mb-8">
                        <CardHeader>
                            <CardTitle>Get Started Today</CardTitle>
                            <CardDescription>Join GitData.ai and revolutionize your ML data workflow</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form className="space-y-4">
                                <Input onChange={(e) => setEmail(e.target.value)} type="email" placeholder="Enter your email address" className="w-full" />
                                <Button
                                    onClick={()=>{
                                        if (Email.length > 0) {
                                            nav("/auth/signup?email="+ Email)
                                        } else {
                                            nav("/auth/signup")
                                        }
                                    }}
                                    type="button"
                                    className="w-full">
                                    Create Account
                                </Button>
                            </form>
                            <div className="mt-4 text-center">
                <span className="text-sm text-muted-foreground">
                  Already have an account?{" "}
                    <NavLink to="/auth/login" className="text-primary hover:underline">
                    Sign in
                  </NavLink>
                </span>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </section>
            <section className="py-16 px-4 sm:px-6 lg:px-8 bg-muted/50">
                <div className="container mx-auto max-w-6xl">
                    <h2 className="text-3xl font-bold text-center mb-12">Powerful Features for ML Data Management</h2>
                    <div className="grid md:grid-cols-3 gap-8">
                        <Card>
                            <CardHeader>
                                <CardTitle className="flex items-center gap-2">
                                    <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                                        <span className="text-primary-foreground font-bold">J</span>
                                    </div>
                                    JiaoZiFS
                                </CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    The first domestic open-source data-centric version control file system. Manage ML datasets with
                                    Git-like operations and ensure model reproducibility.
                                </p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle className="flex items-center gap-2">
                                    <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                                        <span className="text-primary-foreground font-bold">V</span>
                                    </div>
                                    Version Control
                                </CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Track changes, manage versions, and control access to your ML datasets. Reproduce results with
                                    specific dataset versions.
                                </p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle className="flex items-center gap-2">
                                    <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                                        <span className="text-primary-foreground font-bold">A</span>
                                    </div>
                                    Data Analysis
                                </CardTitle>
                            </CardHeader>
                            <CardContent>
                                <p className="text-muted-foreground">
                                    Extract insights from your data with built-in analysis tools. Visualize distributions, relationships,
                                    and generate comprehensive statistics.
                                </p>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </section>
        </>
    )
}