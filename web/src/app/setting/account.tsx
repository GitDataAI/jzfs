import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator
} from "@/components/ui/breadcrumb.tsx";

import { useState } from "react"
import { SettingsHeader } from "@/components/setting/header"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Badge } from "@/components/ui/badge"
import { Save, Shield, AlertTriangle, Trash2, Mail, User, Lock } from "lucide-react"
import {toast} from "sonner";

export const UserSettingAccountPage = () => {
    const [isLoading, setIsLoading] = useState(false)
    const [showPasswordFields, setShowPasswordFields] = useState(false)
    const [showEmailFields, setShowEmailFields] = useState(false)

    const [account, setAccount] = useState({
        username: "johndoe",
        email: "john.doe@example.com",
        emailVerified: true,
        currentPassword: "",
        newPassword: "",
        confirmPassword: "",
        newEmail: "",
    })
    const [deleteConfirmation, setDeleteConfirmation] = useState("")

    const handleUsernameUpdate = async () => {
        setIsLoading(true)
        try {
            await new Promise((resolve) => setTimeout(resolve, 1000))
            toast.success("Username updated");
        } catch (error) {
            console.error(error)
        } finally {
            setIsLoading(false)
        }
    }

    const handlePasswordUpdate = async () => {
        if (account.newPassword !== account.confirmPassword) {
            toast.error("Password mismatch")
            return
        }

        setIsLoading(true)
        try {
            await new Promise((resolve) => setTimeout(resolve, 1000))
            toast.success("Password updated");
            setAccount((prev) => ({ ...prev, currentPassword: "", newPassword: "", confirmPassword: "" }))
            setShowPasswordFields(false)
        } catch (error) {
            console.error(error)
        } finally {
            setIsLoading(false)
        }
    }

    const handleEmailUpdate = async () => {
        setIsLoading(true)
        try {
            await new Promise((resolve) => setTimeout(resolve, 1000))
            toast.success("Email updated");
            setAccount((prev) => ({ ...prev, newEmail: "" }))
            setShowEmailFields(false)
        } catch (error) {
            toast.error("Failed to update email")
        } finally {
            setIsLoading(false)
        }
    }

    const handleAccountDeletion = async () => {
        if (deleteConfirmation !== "DELETE") {
            toast.error("Invalid confirmation")
            return
        }

        setIsLoading(true)
        try {
            await new Promise((resolve) => setTimeout(resolve, 2000))
            toast.success("Account deleted")
        } catch (error) {
            console.error(error)
        } finally {
            setIsLoading(false)
        }
    }
    return(
        <>
            <header className="flex h-16 shrink-0 items-center gap-2 px-4 mt-1">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem className="hidden md:block">
                            <BreadcrumbLink href="#">
                                Settings
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator className="hidden md:block" />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Account</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>
            <div className="max-w-4xl  shrink-0 items-center gap-2 px-4 mt-1">
                <SettingsHeader title="Account Settings" description="Manage your account security and authentication" />

                <div className="space-y-6">
                    {/* Username Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="flex items-center gap-2">
                                <User className="h-5 w-5" />
                                Username
                            </CardTitle>
                            <CardDescription>Change your unique username identifier</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="space-y-2">
                                <Label htmlFor="username">Username</Label>
                                <div className="flex gap-2">
                                    <Input
                                        id="username"
                                        value={account.username}
                                        onChange={(e) => setAccount((prev) => ({ ...prev, username: e.target.value }))}
                                        placeholder="Enter new username"
                                        className="flex-1"
                                    />
                                    <Button onClick={handleUsernameUpdate} disabled={isLoading} variant="outline">
                                        <Save className="h-4 w-4 mr-2" />
                                        Update
                                    </Button>
                                </div>
                                <p className="text-xs text-muted-foreground">
                                    Username must be unique and can only contain letters, numbers, and underscores.
                                </p>
                            </div>
                        </CardContent>
                    </Card>

                    {/* Email Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="flex items-center gap-2">
                                <Mail className="h-5 w-5" />
                                Email Address
                            </CardTitle>
                            <CardDescription>Manage your email address and verification status</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-1">
                                    <p className="text-sm font-medium">{account.email}</p>
                                    <div className="flex items-center gap-2">
                                        {account.emailVerified ? (
                                            <Badge variant="default" className="text-xs">
                                                <Shield className="h-3 w-3 mr-1" />
                                                Verified
                                            </Badge>
                                        ) : (
                                            <Badge variant="destructive" className="text-xs">
                                                <AlertTriangle className="h-3 w-3 mr-1" />
                                                Unverified
                                            </Badge>
                                        )}
                                    </div>
                                </div>
                                <Button variant="outline" onClick={() => setShowEmailFields(!showEmailFields)}>
                                    Change Email
                                </Button>
                            </div>

                            {showEmailFields && (
                                <div className="space-y-4 pt-4 border-t">
                                    <div className="space-y-2">
                                        <Label htmlFor="newEmail">New Email Address</Label>
                                        <Input
                                            id="newEmail"
                                            type="email"
                                            value={account.newEmail}
                                            onChange={(e) => setAccount((prev) => ({ ...prev, newEmail: e.target.value }))}
                                            placeholder="Enter new email address"
                                        />
                                    </div>
                                    <div className="flex gap-2">
                                        <Button onClick={handleEmailUpdate} disabled={isLoading}>
                                            Send Verification
                                        </Button>
                                        <Button variant="outline" onClick={() => setShowEmailFields(false)}>
                                            Cancel
                                        </Button>
                                    </div>
                                </div>
                            )}
                        </CardContent>
                    </Card>

                    {/* Password Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="flex items-center gap-2">
                                <Lock className="h-5 w-5" />
                                Password
                            </CardTitle>
                            <CardDescription>Update your account password for better security</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-1">
                                    <p className="text-sm font-medium">Password</p>
                                    <p className="text-xs text-muted-foreground">Last updated 30 days ago</p>
                                </div>
                                <Button variant="outline" onClick={() => setShowPasswordFields(!showPasswordFields)}>
                                    Change Password
                                </Button>
                            </div>

                            {showPasswordFields && (
                                <div className="space-y-4 pt-4 border-t">
                                    <div className="space-y-2">
                                        <Label htmlFor="currentPassword">Current Password</Label>
                                        <Input
                                            id="currentPassword"
                                            type="password"
                                            value={account.currentPassword}
                                            onChange={(e) => setAccount((prev) => ({ ...prev, currentPassword: e.target.value }))}
                                            placeholder="Enter current password"
                                        />
                                    </div>
                                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <div className="space-y-2">
                                            <Label htmlFor="newPassword">New Password</Label>
                                            <Input
                                                id="newPassword"
                                                type="password"
                                                value={account.newPassword}
                                                onChange={(e) => setAccount((prev) => ({ ...prev, newPassword: e.target.value }))}
                                                placeholder="Enter new password"
                                            />
                                        </div>
                                        <div className="space-y-2">
                                            <Label htmlFor="confirmPassword">Confirm New Password</Label>
                                            <Input
                                                id="confirmPassword"
                                                type="password"
                                                value={account.confirmPassword}
                                                onChange={(e) => setAccount((prev) => ({ ...prev, confirmPassword: e.target.value }))}
                                                placeholder="Confirm new password"
                                            />
                                        </div>
                                    </div>
                                    <div className="flex gap-2">
                                        <Button onClick={handlePasswordUpdate} disabled={isLoading}>
                                            Update Password
                                        </Button>
                                        <Button variant="outline" onClick={() => setShowPasswordFields(false)}>
                                            Cancel
                                        </Button>
                                    </div>
                                </div>
                            )}
                        </CardContent>
                    </Card>

                    {/* Danger Zone */}
                    <Card className="border-destructive">
                        <CardHeader>
                            <CardTitle className="flex items-center gap-2 text-destructive">
                                <AlertTriangle className="h-5 w-5" />
                                Danger Zone
                            </CardTitle>
                            <CardDescription>Irreversible and destructive actions</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="p-4 border border-destructive/20 rounded-lg bg-destructive/5">
                                <div className="flex items-start justify-between">
                                    <div className="space-y-1">
                                        <h4 className="text-sm font-medium text-destructive">Delete Account</h4>
                                        <p className="text-xs text-muted-foreground">
                                            Permanently delete your account and all associated data. This action cannot be undone.
                                        </p>
                                    </div>
                                    <AlertDialog>
                                        <AlertDialogTrigger asChild>
                                            <Button variant="destructive" size="sm">
                                                <Trash2 className="h-4 w-4 mr-2" />
                                                Delete Account
                                            </Button>
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle className="flex items-center gap-2 text-destructive">
                                                    <AlertTriangle className="h-5 w-5" />
                                                    Delete Account
                                                </AlertDialogTitle>
                                                <AlertDialogDescription className="space-y-3">
                                                    <p>This action will permanently delete your account and all associated data including:</p>
                                                    <ul className="list-disc list-inside space-y-1 text-sm">
                                                        <li>Profile information and settings</li>
                                                        <li>All projects and repositories</li>
                                                        <li>SSH keys and access tokens</li>
                                                        <li>Account history and activity logs</li>
                                                    </ul>
                                                    <p className="font-medium">
                                                        This action cannot be undone. Please type <strong>DELETE</strong> to confirm.
                                                    </p>
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <div className="space-y-2">
                                                <Label htmlFor="deleteConfirmation">Type DELETE to confirm</Label>
                                                <Input
                                                    id="deleteConfirmation"
                                                    value={deleteConfirmation}
                                                    onChange={(e) => setDeleteConfirmation(e.target.value)}
                                                    placeholder="DELETE"
                                                />
                                            </div>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel onClick={() => setDeleteConfirmation("")}>Cancel</AlertDialogCancel>
                                                <AlertDialogAction
                                                    onClick={handleAccountDeletion}
                                                    className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                                                    disabled={deleteConfirmation !== "DELETE" || isLoading}
                                                >
                                                    {isLoading ? "Deleting..." : "Delete Account"}
                                                </AlertDialogAction>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </>
    )
}