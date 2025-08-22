import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { CheckCircle } from 'lucide-react'

export function ResetPasswordForm({
                                      className,
                                      ...props
                                  }: React.ComponentProps<"div">) {
    return (
        <div className={cn("flex flex-col gap-6", className)} {...props}>
            <Card>
                <CardHeader className="text-center">
                    <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
                        <CheckCircle className="h-6 w-6 text-green-600" />
                    </div>
                    <CardTitle className="text-xl">Reset your password</CardTitle>
                    <CardDescription>
                        Enter your new password below
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <form>
                        <div className="grid gap-6">
                            <div className="grid gap-3">
                                <Label htmlFor="new-password">New Password</Label>
                                <Input id="new-password" type="password" required />
                                <p className="text-xs text-muted-foreground">
                                    Password must be at least 8 characters long
                                </p>
                            </div>
                            <div className="grid gap-3">
                                <Label htmlFor="confirm-new-password">Confirm New Password</Label>
                                <Input id="confirm-new-password" type="password" required />
                            </div>
                            <Button type="submit" className="w-full">
                                Reset Password
                            </Button>
                            <div className="text-center text-sm">
                                <a href="/auth/login" className="underline underline-offset-4">
                                    Back to login
                                </a>
                            </div>
                        </div>
                    </form>
                </CardContent>
            </Card>
        </div>
    )
}
