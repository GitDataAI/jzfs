import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator
} from "@/components/ui/breadcrumb.tsx";
import {useEffect, useState} from "react"
import { SettingsHeader } from "@/components/setting/header"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Camera, Save } from "lucide-react"
import useProfile from "@/hooks/use-profile.tsx";

const languages = [
    { value: "en", label: "English" },
    { value: "zh", label: "中文" },
    { value: "es", label: "Español" },
    { value: "fr", label: "Français" },
    { value: "de", label: "Deutsch" },
    { value: "ja", label: "日本語" },
]

const themes = [
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
    { value: "system", label: "System" },
]
export const UserSettingProfilePage = () => {
    const { profile, isLoading, fetchProfile, updateProfile } = useProfile();
    const [tempProfile, setTempProfile] = useState(profile);
    useEffect(() => {
        fetchProfile();
    }, []);
    useEffect(() => {
        setTempProfile(profile);
    }, [profile]);
    const handleAvatarUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0]
        if (file) {
            const reader = new FileReader()
            reader.onload = (e) => {
                setTempProfile(prev => ({
                    ...prev,
                    avatar: e.target?.result as string
                }));
            }
            reader.readAsDataURL(file)
        }
    }
    const handleSave = async () => {
        try {
            await updateProfile(tempProfile);
        } catch (error) {
            console.error(error)
        }
    }
    return(
        <>
            <header className="flex h-16 shrink-0 items-center gap-2 px-4 mt-1">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem className="hidden md:block">
                            <BreadcrumbLink href="#">
                                Setting
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator className="hidden md:block" />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Profile</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>
            <div className="max-w-4xl shrink-0 items-center gap-2 px-4 mt-1">
                <SettingsHeader title="Profile Settings" description="Manage your profile information and preferences" />

                <div className="space-y-6">
                    {/* Avatar Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Profile Picture</CardTitle>
                            <CardDescription>Upload a profile picture to personalize your account</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <div className="flex items-center gap-6">
                                <div className="relative">
                                    <Avatar className="h-24 w-24">
                                        <AvatarImage src={tempProfile.avatar || "/placeholder.svg"} alt="Profile picture" />
                                        <AvatarFallback className="text-lg">
                                            {tempProfile.display_name || "U" }
                                        </AvatarFallback>
                                    </Avatar>
                                    <label
                                        htmlFor="avatar-upload"
                                        className="absolute -bottom-2 -right-2 bg-primary text-primary-foreground rounded-full p-2 cursor-pointer hover:bg-primary/90 transition-colors"
                                    >
                                        <Camera className="h-4 w-4" />
                                        <input
                                            id="avatar-upload"
                                            type="file"
                                            accept="image/*"
                                            className="hidden"
                                            onChange={handleAvatarUpload}
                                        />
                                    </label>
                                </div>
                                <div>
                                    <p className="text-sm text-muted-foreground">
                                        Click the camera icon to upload a new picture. Recommended size: 400x400px.
                                    </p>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>Personal Information</CardTitle>
                            <CardDescription>Update your personal details and contact information</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div className="space-y-2">
                                    <Label htmlFor="displayName">Display Name</Label>
                                    <Input
                                    id="displayName"
                                    value={tempProfile.display_name}
                                    onChange={(e) => setTempProfile(prev => ({ ...prev, display_name: e.target.value }))}
                                    placeholder="Enter your display name"
                                />
                                </div>
                                <div className="space-y-2">
                                    <Label htmlFor="location">Location</Label>
                                    <Input
                                    id="location"
                                    value={tempProfile.location}
                                    onChange={(e) => setTempProfile(prev => ({ ...prev, location: e.target.value }))}
                                    placeholder="Enter your location"
                                />
                                </div>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="company">Company</Label>
                                <Input
                                    id="company"
                                    value={tempProfile.company}
                                    onChange={(e) => setTempProfile(prev => ({ ...prev, company: e.target.value }))}
                                    placeholder="Enter your company"
                                />
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="signature">Bio/Signature</Label>
                                <Textarea
                                    id="signature"
                                    value={tempProfile.bio}
                                    onChange={(e) => setTempProfile(prev => ({ ...prev, bio: e.target.value }))}
                                    placeholder="Tell us about yourself..."
                                    rows={3}
                                />
                                <p className="text-xs text-muted-foreground">{tempProfile.bio?.length || 0}/500 characters</p>
                            </div>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>Preferences</CardTitle>
                            <CardDescription>Customize your experience with language and theme settings</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div className="space-y-2">
                                    <Label htmlFor="language">Language</Label>
                                    <Select
                                         value={tempProfile.language}
                                         onValueChange={(value) => setTempProfile(prev => ({ ...prev, language: value }))}
                                    >
                                        <SelectTrigger>
                                            <SelectValue placeholder="Select language" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            {languages.map((lang) => (
                                                <SelectItem key={lang.value} value={lang.value}>
                                                    {lang.label}
                                                </SelectItem>
                                            ))}
                                        </SelectContent>
                                    </Select>
                                </div>

                                <div className="space-y-2">
                                    <Label htmlFor="theme">Theme</Label>
                                    <Select
                                         value={tempProfile.theme}
                                         onValueChange={(value) => setTempProfile(prev => ({ ...prev, theme: value }))}
                                    >
                                        <SelectTrigger>
                                            <SelectValue placeholder="Select theme" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            {themes.map((theme) => (
                                                <SelectItem key={theme.value} value={theme.value}>
                                                    {theme.label}
                                                </SelectItem>
                                            ))}
                                        </SelectContent>
                                    </Select>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <div className="flex justify-end">
                        <Button onClick={handleSave} disabled={isLoading} className="min-w-32">
                            {isLoading ? (
                                <div className="flex items-center gap-2">
                                    <div className="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent" />
                                    Saving...
                                </div>
                            ) : (
                                <div className="flex items-center gap-2">
                                    <Save className="h-4 w-4" />
                                    Save Changes
                                </div>
                            )}
                        </Button>
                    </div>
                </div>
            </div>
        </>
    )
}