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
import { Checkbox } from '@/components/ui/checkbox'
import * as React from "react";
import {useState} from "react";
import {useEmailCaptcha} from "@/hooks/use-email-captcha.tsx";
import {toast} from "sonner";
import { useAuths } from "@/hooks/use-auths.tsx";


export function SignupForm({className, ...props}: React.ComponentProps<"div">) {
    const [Username, setUsername] = useState<string>("");
    const [Email, setEmail] = useState<string>("");
    const [Password, setPassword] = useState<string>("");
    const [ConfirmPassword, setConfirmPassword] = useState<string>("");
    const [Agree, setAgree] = useState<boolean>(false);
    const [EmailCode, setEmailCode] = useState<string>("");
    const captcha = useEmailCaptcha();
    const user = useAuths();
    const handleSendEmailCaptcha = async () => {
        if (Email.trim() === "") {
            toast.warning("Please enter your email")
            return;
        }
        if (!Email.includes("@")) {
            toast.warning("Please enter a valid email")
            return;
        }
        const result = await captcha.sendEmailCaptcha(Email);
        console.log(result);
        if (result.code === 200) {
            toast.success("Email sent successfully")
        } else {
            toast.error(result.msg)
        }
    };
    const handleVerfyEmailCaptcha = async () => {
        const result = await captcha.verifyEmailCaptcha(Email, EmailCode);
        if (result.code === 200) {
            toast.success("Email verification successful")
            return true;
        } else {
            toast.error(result.msg)
            return false;
        }
    };
    const handleRegisterBefore = async () => {
        if (Email.trim() === "") {
            toast.warning("Please enter your email")
            return;
        }
        if (!Email.includes("@")) {
            toast.warning("Please enter a valid email")
            return;
        }
        if (!await user.register_after(Username,Email)) {
            toast.warning("This email or username has been registered");
            return false;
        } else {
            return true;
        }
    }
    const handleRegister = async () => {

        if (Username.trim() === "") {
            toast.warning("Please enter your username")
            return;
        }
        if (Email.trim() === "") {
            toast.warning("Please enter your email")
            return;
        }
        if (Password.trim() === "") {
            toast.warning("Please enter your password")
            return;
        }
        if (ConfirmPassword.trim() === "") {
            toast.warning("Please enter your confirm password")
        }
        if (Password !== ConfirmPassword) {
            toast.warning("Password and confirm password are not the same")
            return;
        }
        if (!Agree) {
            toast.warning("Please agree to the terms of service")
            return;
        }
        if (!await handleRegisterBefore()) {
            return;
        }
        if (!await handleVerfyEmailCaptcha()) {
            return;
        }
        user.register(Username, Email, Password).then(() => {
            toast.success("Register successful")
            window.location.href = "/auth/login";
        }).catch((e) => {
            toast.error(e.msg)
        })
    }
    return (
        <div className={cn("flex flex-col gap-6", className)} {...props}>
            <Card>
                <CardHeader className="text-center">
                    <CardTitle className="text-xl">Create an account</CardTitle>
                    <CardDescription>
                        Enter your information to create an account
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <form>
                        <div className="grid gap-6">
                            {/*<div className="flex flex-col gap-4">*/}
                            {/*    <Button variant="outline" className="w-full">*/}
                            {/*        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">*/}
                            {/*            <path*/}
                            {/*                d="M12.152 6.896c-.948 0-2.415-1.078-3.96-1.04-2.04.027-3.91 1.183-4.961 3.014-2.117 3.675-.546 9.103 1.519 12.09 1.013 1.454 2.208 3.09 3.792 3.039 1.52-.065 2.09-.987 3.935-.987 1.831 0 2.35.987 3.96.948 1.637-.026 2.676-1.48 3.676-2.948 1.156-1.688 1.636-3.325 1.662-3.415-.039-.013-3.182-1.221-3.22-4.857-.026-3.04 2.48-4.494 2.597-4.559-1.429-2.09-3.623-2.324-4.39-2.376-2-.156-3.675 1.09-4.61 1.09zM15.53 3.83c.843-1.012 1.4-2.427 1.245-3.83-1.207.052-2.662.805-3.532 1.818-.78.896-1.454 2.338-1.273 3.714 1.338.104 2.715-.688 3.559-1.701"*/}
                            {/*                fill="currentColor"*/}
                            {/*            />*/}
                            {/*        </svg>*/}
                            {/*        Sign up with Github*/}
                            {/*    </Button>*/}
                            {/*    <Button variant="outline" className="w-full">*/}
                            {/*        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">*/}
                            {/*            <path*/}
                            {/*                d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"*/}
                            {/*                fill="currentColor"*/}
                            {/*            />*/}
                            {/*        </svg>*/}
                            {/*        Sign up with Google*/}
                            {/*    </Button>*/}
                            {/*</div>*/}
                            {/*<div className="after:border-border relative text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">*/}
                            {/*<span className="bg-card text-muted-foreground relative z-10 px-2">*/}
                            {/*  Or continue with*/}
                            {/*</span>*/}
                            {/*</div>*/}
                            <div className="grid gap-6">
                                <div className="grid gap-4">
                                    <div className="grid gap-3">
                                        <Label htmlFor="username">username</Label>
                                        <Input id="username" placeholder="Max" required onChange={(e) => setUsername(e.target.value)}/>
                                    </div>
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="email">Email</Label>
                                    <Input
                                        id="email"
                                        type="email"
                                        placeholder="m@example.com"
                                        required
                                        onChange={(e) => setEmail(e.target.value)}
                                    />
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="password">Password</Label>
                                    <Input id="password" type="password" required onChange={(e) => setPassword(e.target.value)}/>
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="confirm-password">Confirm Password</Label>
                                    <Input id="confirm-password" type="password" required onChange={(e) => setConfirmPassword(e.target.value)}/>
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="email-code">Email Code</Label>
                                    <div className="flex gap-4">
                                        <Input id="email-code" type="text" required onChange={(e) => setEmailCode(e.target.value)}/>
                                        <Button className="w-1/3" onClick={handleSendEmailCaptcha}>Send</Button>
                                    </div>
                                </div>
                                <div className="flex items-center space-x-2">
                                    <Checkbox id="terms" onClick={()=> setAgree(!Agree)}/>
                                    <label
                                        htmlFor="terms"
                                        className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                    >
                                        I agree to the{" "}
                                        <a href="#" className="underline underline-offset-4">
                                            Terms of Service
                                        </a>{" "}
                                        and{" "}
                                        <a href="#" className="underline underline-offset-4">
                                            Privacy Policy
                                        </a>
                                    </label>
                                </div>
                                <Button type="button" className="w-full" onClick={handleRegister}>
                                    Create account
                                </Button>
                            </div>
                            <div className="text-center text-sm">
                                Already have an account?{" "}
                                <a href="/auth/login" className="underline underline-offset-4">
                                    Sign in
                                </a>
                            </div>
                        </div>
                    </form>
                </CardContent>
            </Card>
        </div>
    )
}
