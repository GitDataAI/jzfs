'use client'

import { useState } from 'react'
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
import { Shield } from 'lucide-react'

export function TwoFactorForm({
                                  className,
                                  ...props
                              }: React.ComponentProps<"div">) {
    const [code, setCode] = useState(['', '', '', '', '', ''])

    const handleInputChange = (index: number, value: string) => {
        if (value.length <= 1) {
            const newCode = [...code]
            newCode[index] = value
            setCode(newCode)

            // Auto-focus next input
            if (value && index < 5) {
                const nextInput = document.getElementById(`code-${index + 1}`)
                nextInput?.focus()
            }
        }
    }

    const handleKeyDown = (index: number, e: React.KeyboardEvent) => {
        if (e.key === 'Backspace' && !code[index] && index > 0) {
            const prevInput = document.getElementById(`code-${index - 1}`)
            prevInput?.focus()
        }
    }

    return (
        <div className={cn("flex flex-col gap-6", className)} {...props}>
            <Card>
                <CardHeader className="text-center">
                    <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-blue-100">
                        <Shield className="h-6 w-6 text-blue-600" />
                    </div>
                    <CardTitle className="text-xl">Two-Factor Authentication</CardTitle>
                    <CardDescription>
                        Enter the 6-digit code from your authenticator app
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <form>
                        <div className="grid gap-6">
                            <div className="grid gap-3">
                                <Label htmlFor="code-0">Verification Code</Label>
                                <div className="flex gap-2 justify-center">
                                    {code.map((digit, index) => (
                                        <Input
                                            key={index}
                                            id={`code-${index}`}
                                            type="text"
                                            inputMode="numeric"
                                            maxLength={1}
                                            value={digit}
                                            onChange={(e) => handleInputChange(index, e.target.value)}
                                            onKeyDown={(e) => handleKeyDown(index, e)}
                                            className="w-12 h-12 text-center text-lg font-semibold"
                                            autoComplete="off"
                                        />
                                    ))}
                                </div>
                            </div>
                            <Button type="submit" className="w-full">
                                Verify Code
                            </Button>
                            <div className="text-center text-sm">
                                <p className="text-muted-foreground mb-2">
                                    Didn't receive a code?
                                </p>
                                <Button variant="link" className="p-0 h-auto">
                                    Resend code
                                </Button>
                            </div>
                        </div>
                    </form>
                </CardContent>
            </Card>
        </div>
    )
}
