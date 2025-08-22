import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator
} from "@/components/ui/breadcrumb.tsx";
import { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import useAccessKeys from "@/hooks/use-access-keys"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Badge } from "@/components/ui/badge"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
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
import { SettingsHeader } from "@/components/setting/header"
import { Plus, Trash2, Edit } from "lucide-react"
import { Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious } from "@/components/ui/pagination.tsx"
import {toast} from "sonner"

// AccessKey interface is defined in use-access-keys hook

const PERMISSION_LABELS = {
    repo_access: "Repository Access",
    email_access: "Email Access",
    event_access: "Event Access",
    follow_access: "Follow Access",
    gpg_access: "GPG Access",
    ssh_access: "SSH Access",
    webhook_access: "Webhook Access",
    wiki_access: "Wiki Access",
    project_access: "Project Access",
    issue_access: "Issue Access",
    comment_access: "Comment Access",
    profile_access: "Profile Access",
}

const ACCESS_LEVELS = [
    { value: 0, label: "No Access", color: "secondary" },
    { value: 1, label: "Read", color: "default" },
    { value: 2, label: "Read & Write", color: "destructive" },
]

export const UserSettingAccessKeyPage = () => {
    const { accessKeys, currentPage, pageSize, totalCount, fetchAccessKeys, createAccessKey, deleteAccessKey } = useAccessKeys()

    const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
    const [newKey, setNewKey] = useState({
        title: "",
        description: "",
        expiration: "",
        repo_access: 0,
        email_access: 0,
        event_access: 0,
        follow_access: 0,
        gpg_access: 0,
        ssh_access: 0,
        webhook_access: 0,
        wiki_access: 0,
        project_access: 0,
        issue_access: 0,
        comment_access: 0,
        profile_access: 0,
    })

    useEffect(() => {
        fetchAccessKeys(currentPage, pageSize)
    }, [currentPage, pageSize])

    const handlePageChange = (page: number) => {
        fetchAccessKeys(page, pageSize)
    }

    const handleCreateKey = async () => {
        if (!newKey.title || !newKey.expiration) {
            toast.error("Title and expiration date are required.")
            return
        }

        await createAccessKey(newKey)
        setNewKey({
            title: "",
            description: "",
            expiration: "",
            repo_access: 0,
            email_access: 0,
            event_access: 0,
            follow_access: 0,
            gpg_access: 0,
            ssh_access: 0,
            webhook_access: 0,
            wiki_access: 0,
            project_access: 0,
            issue_access: 0,
            comment_access: 0,
            profile_access: 0,
        })
        setIsCreateDialogOpen(false)
    }

    const handleDeleteKey = async (id: string) => {
        await deleteAccessKey(id)
    }

    const getAccessLevelBadge = (level: number) => {
        const accessLevel = ACCESS_LEVELS.find((l) => l.value === level)
        return (
            <Badge variant={accessLevel?.color as any} className="text-xs">
                {accessLevel?.label}
            </Badge>
        )
    }

    const isExpired = (expiration: string) => {
        return new Date(expiration) < new Date()
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
                            <BreadcrumbPage>AccessKey</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>
            <div className="space-y-6 shrink-0 items-center gap-2 px-4 mt-1">
                <SettingsHeader
                    title="Access Keys"
                    description="Manage API access keys and their permissions for external integrations."
                />

                <div className="flex justify-between items-center">
                    <div>
                        <h3 className="text-lg font-medium">Your Access Keys</h3>
                        <p className="text-sm text-muted-foreground">
                            {accessKeys?.length || 0} access key{(accessKeys?.length || 0) !== 1 ? "s" : ""} configured
                        </p>
                    </div>

                    <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
                        <DialogTrigger asChild>
                            <Button>
                                <Plus className="h-4 w-4 mr-2" />
                                Create Access Key
                            </Button>
                        </DialogTrigger>
                        <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
                            <DialogHeader>
                                <DialogTitle>Create New Access Key</DialogTitle>
                                <DialogDescription>
                                    Create a new access key with specific permissions for external integrations.
                                </DialogDescription>
                            </DialogHeader>

                            <div className="space-y-4">
                                <div className="grid grid-cols-2 gap-4">
                                    <div className="space-y-2">
                                        <Label htmlFor="title">Title *</Label>
                                        <Input
                                            id="title"
                                            value={newKey.title}
                                            onChange={(e) => setNewKey({ ...newKey, title: e.target.value })}
                                            placeholder="e.g., GitHub Actions CI/CD"
                                        />
                                    </div>
                                    <div className="space-y-2">
                                        <Label htmlFor="expiration">Expiration Date *</Label>
                                        <Input
                                            id="expiration"
                                            type="date"
                                            value={newKey.expiration}
                                            onChange={(e) => setNewKey({ ...newKey, expiration: e.target.value })}
                                        />
                                    </div>
                                </div>

                                <div className="space-y-2">
                                    <Label htmlFor="description">Description</Label>
                                    <Textarea
                                        id="description"
                                        value={newKey.description}
                                        onChange={(e) => setNewKey({ ...newKey, description: e.target.value })}
                                        placeholder="Optional description of what this key is used for"
                                        rows={2}
                                    />
                                </div>

                                <div className="space-y-4">
                                    <h4 className="font-medium">Permissions</h4>
                                    <div className="grid grid-cols-2 gap-4">
                                        {Object.entries(PERMISSION_LABELS).map(([key, label]) => (
                                            <div key={key} className="space-y-2">
                                                <Label>{label}</Label>
                                                <Select
                                                    value={newKey[key as keyof typeof newKey].toString()}
                                                    onValueChange={(value) => setNewKey({ ...newKey, [key]: Number.parseInt(value) })}
                                                >
                                                    <SelectTrigger>
                                                        <SelectValue />
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        {ACCESS_LEVELS.map((level) => (
                                                            <SelectItem key={level.value} value={level.value.toString()}>
                                                                {level.label}
                                                            </SelectItem>
                                                        ))}
                                                    </SelectContent>
                                                </Select>
                                            </div>
                                        ))}
                                    </div>
                                </div>
                            </div>

                            <DialogFooter>
                                <Button variant="outline" onClick={() => setIsCreateDialogOpen(false)}>
                                    Cancel
                                </Button>
                                <Button onClick={handleCreateKey}>Create Access Key</Button>
                            </DialogFooter>
                        </DialogContent>
                    </Dialog>
                </div>

                <div className="space-y-4">
                    {!accessKeys || accessKeys.length === 0 ? (
                        <Card>
                            <CardContent className="flex flex-col items-center justify-center py-12">
                                <div className="text-center space-y-2">
                                    <h3 className="text-lg font-medium">No access keys</h3>
                                    <p className="text-sm text-muted-foreground">Create your first access key to start using the API.</p>
                                </div>
                            </CardContent>
                        </Card>
                    ) : (
                        accessKeys.map((key) => (
                            <Card key={key.id} className={isExpired(key.expiration) ? "border-destructive" : ""}>
                                <CardHeader>
                                    <div className="flex items-start justify-between">
                                        <div className="space-y-1">
                                            <div className="flex items-center gap-2">
                                                <CardTitle className="text-lg">{key.title}</CardTitle>
                                                {isExpired(key.expiration) && <Badge variant="destructive">Expired</Badge>}
                                            </div>
                                            {key.description && <CardDescription>{key.description}</CardDescription>}
                                        </div>
                                        <div className="flex items-center gap-2">
                                            <Button variant="outline" size="sm">
                                                <Edit className="h-4 w-4" />
                                            </Button>
                                            <AlertDialog>
                                                <AlertDialogTrigger asChild>
                                                    <Button variant="outline" size="sm">
                                                        <Trash2 className="h-4 w-4" />
                                                    </Button>
                                                </AlertDialogTrigger>
                                                <AlertDialogContent>
                                                    <AlertDialogHeader>
                                                        <AlertDialogTitle>Delete Access Key</AlertDialogTitle>
                                                        <AlertDialogDescription>
                                                            Are you sure you want to delete "{key.title}"? This action cannot be undone and will
                                                            immediately revoke access for any applications using this key.
                                                        </AlertDialogDescription>
                                                    </AlertDialogHeader>
                                                    <AlertDialogFooter>
                                                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                        <AlertDialogAction
                                                            onClick={() => handleDeleteKey(key.id)}
                                                            className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                                                        >
                                                            Delete Key
                                                        </AlertDialogAction>
                                                    </AlertDialogFooter>
                                                </AlertDialogContent>
                                            </AlertDialog>
                                        </div>
                                    </div>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div className="grid grid-cols-1 gap-4 text-sm">
                                        <div>
                                            <span className="text-muted-foreground">Created:</span> {key.created_at}
                                        </div>
                                        <div>
                                            <span className="text-muted-foreground">Expires:</span> {key.expiration}
                                        </div>
                                        <div>
                                            <span className="text-muted-foreground">Last used:</span> {key.last_used || "Never"}
                                        </div>
                                    </div>

                                    <div className="space-y-2">
                                        <Label>Permissions</Label>
                                        <div className="grid grid-cols-3 gap-2">
                                            {Object.entries(PERMISSION_LABELS).map(([permKey, label]) => {
                                                const level = (key as any)[permKey] as number
                                                if (level === 0) return null
                                                return (
                                                    <div key={permKey} className="flex items-center justify-between text-sm">
                                                        <span>{label}</span>
                                                        {getAccessLevelBadge(level)}
                                                    </div>
                                                )
                                            })}
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        ))
                    )}
                </div>

                <div className="mt-8 flex justify-center">
                    <Pagination>
                        <PaginationContent>
                            <PaginationItem>
                                <PaginationPrevious
                                    onClick={(e) => {
                                        e.preventDefault();
                                        if (currentPage > 1) {
                                            handlePageChange(currentPage - 1);
                                        }
                                    }}
                                    aria-disabled={currentPage === 1}
                                />
                            </PaginationItem>

                            {Array.from({ length: Math.ceil(totalCount / pageSize) }, (_, i) => i + 1).map((page) => (
                                <PaginationItem key={page}>
                                    <PaginationLink
                                        isActive={page === currentPage}
                                        onClick={(e) => {
                                            e.preventDefault();
                                            handlePageChange(page);
                                        }}
                                    >
                                        {page}
                                    </PaginationLink>
                                </PaginationItem>
                            ))}

                            <PaginationItem>
                                <PaginationNext
                                    onClick={(e) => {
                                        e.preventDefault();
                                        if (currentPage < Math.ceil(totalCount / pageSize)) {
                                            handlePageChange(currentPage + 1);
                                        }
                                    }}
                                    aria-disabled={currentPage >= Math.ceil(totalCount / pageSize)}
                                />
                            </PaginationItem>
                        </PaginationContent>
                    </Pagination>
                </div>
            </div>
        </>
    )
}