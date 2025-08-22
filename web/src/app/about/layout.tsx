import { Button } from "@/components/ui/button"

import {NavLink, Outlet} from "react-router-dom";

export default function AboutLayout() {
    return (
        <div className="min-h-screen bg-background">
            {/* Header Navigation */}
            <header className="border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <div className="container mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="flex h-16 items-center justify-between">
                        <div className="flex items-center">
                            <NavLink to="/about" className="text-2xl font-bold text-primary">
                                GitData.ai
                            </NavLink>
                        </div>
                        <nav className="hidden md:flex items-center space-x-8">
                            <NavLink to="/about/project" className="text-foreground/80 hover:text-foreground transition-colors">
                                Project
                            </NavLink>
                            <NavLink to="/about/solutions" className="text-foreground/80 hover:text-foreground transition-colors">
                                Solutions
                            </NavLink>
                            <NavLink to="/about/resources" className="text-foreground/80 hover:text-foreground transition-colors">
                                Resources
                            </NavLink>
                            <NavLink
                                to="https://docs.gitdata.ai"
                                target="_blank"
                                className="text-foreground/80 hover:text-foreground transition-colors"
                            >
                                Docs
                            </NavLink>
                            <NavLink to="/about/pricing" className="text-foreground/80 hover:text-foreground transition-colors">
                                Price
                            </NavLink>
                        </nav>
                        <div className="flex items-center space-x-4">
                            <Button variant="ghost" asChild>
                                <NavLink to="/auth/login">Login</NavLink>
                            </Button>
                            <Button asChild>
                                <NavLink to="/auth/signup">Sign Up</NavLink>
                            </Button>
                        </div>
                    </div>
                </div>
            </header>
            <Outlet/>

            {/* Footer */}
            <footer className="bg-background border-t border-border py-12 px-4 sm:px-6 lg:px-8">
                <div className="container mx-auto max-w-6xl">
                    <div className="grid md:grid-cols-4 gap-8">
                        <div>
                            <h3 className="font-bold text-lg mb-4">GitData.ai</h3>
                            <p className="text-muted-foreground text-sm">
                                Revolutionizing ML data management with Git-inspired version control.
                            </p>
                        </div>
                        <div>
                            <h4 className="font-semibold mb-4">Product</h4>
                            <ul className="space-y-2 text-sm">
                                <li>
                                    <NavLink to="/features" className="text-muted-foreground hover:text-foreground">
                                        Features
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/pricing" className="text-muted-foreground hover:text-foreground">
                                        Pricing
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/integrations" className="text-muted-foreground hover:text-foreground">
                                        Integrations
                                    </NavLink>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h4 className="font-semibold mb-4">Resources</h4>
                            <ul className="space-y-2 text-sm">
                                <li>
                                    <NavLink to="/docs" className="text-muted-foreground hover:text-foreground">
                                        Documentation
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/blog" className="text-muted-foreground hover:text-foreground">
                                        Blog
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/support" className="text-muted-foreground hover:text-foreground">
                                        Support
                                    </NavLink>
                                </li>
                            </ul>
                        </div>
                        <div>
                            <h4 className="font-semibold mb-4">Company</h4>
                            <ul className="space-y-2 text-sm">
                                <li>
                                    <NavLink to="/about" className="text-muted-foreground hover:text-foreground">
                                        About Us
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/careers" className="text-muted-foreground hover:text-foreground">
                                        Careers
                                    </NavLink>
                                </li>
                                <li>
                                    <NavLink to="/contact" className="text-muted-foreground hover:text-foreground">
                                        Contact
                                    </NavLink>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div className="border-t border-border mt-8 pt-8 text-center text-sm text-muted-foreground">
                        <p>&copy; 2024 GitData.ai. All rights reserved.</p>
                    </div>
                </div>
            </footer>
        </div>
    )
}
