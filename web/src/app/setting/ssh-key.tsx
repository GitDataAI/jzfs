import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator
} from "@/components/ui/breadcrumb.tsx";

import { useState, useEffect } from "react"
import { SettingsHeader } from "@/components/setting/header"
import useSSHKeys from "@/hooks/use-ssh-keys"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
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
import { Plus, Key, Trash2, Copy, Calendar, Clock, Shield } from "lucide-react"
import { Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious } from "@/components/ui/pagination.tsx"
import {toast} from "sonner";


export const UserSettingSSHKeyPage = () => {
    const { sshKeys, isLoading, currentPage, pageSize, totalCount, fetchSSHKeys, addSSHKey, deleteSSHKey } = useSSHKeys()
    const [isAddDialogOpen, setIsAddDialogOpen] = useState(false)
    const [newKey, setNewKey] = useState({
        title: "",
        key: "",
    })
    useEffect(() => {
        fetchSSHKeys(currentPage, pageSize)
    }, []);

    useEffect(() => {
        fetchSSHKeys(currentPage, pageSize)
    }, [currentPage, pageSize])

    const handlePageChange = (page: number) => {
        fetchSSHKeys(page, pageSize)
    }

    const copyToClipboard = async (text: string) => {
        try {
            await navigator.clipboard.writeText(text)
            toast.success("Copied to clipboard")
        } catch (error) {
            toast.error(`Failed to copy error\`${error}\``)
        }
    }
    const handleAddKey = async () => {
        if (!newKey.title.trim() || !newKey.key.trim()) {
            toast.error("Please fill out all fields.")
            return
        }

        try {
            await addSSHKey(newKey)
            setNewKey({ title: "", key: "" })
            setIsAddDialogOpen(false)
        } catch (error) {
            toast.error(`Failed to add key error\`${error}\``)
        }
    }

    const handleDeleteKey = async (keyId: string) => {
        try {
            await deleteSSHKey(keyId)
        } catch (error) {
            toast.error(`Failed to delete key error\`${error}\``)
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
                            <BreadcrumbPage>SSH Key</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>
            <div className="max-w-4xl shrink-0 items-center gap-2 px-4 mt-1">
                <SettingsHeader title="SSH Keys" description="Manage your SSH keys for secure repository access" />
                <div className="space-y-6">
                    <div className="flex justify-between items-center">
                        <div>
                            <h2 className="text-lg font-semibold">Your SSH Keys</h2>
                        </div>
                        <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
                            <DialogTrigger asChild>
                                <Button>
                                    <Plus className="h-4 w-4 mr-2" />
                                    Add SSH Key
                                </Button>
                            </DialogTrigger>
                            <DialogContent className="max-w-3xl">
                                <DialogHeader className="max-w-full">
                                    <DialogTitle>Add New SSH Key</DialogTitle>
                                    <DialogDescription>
                                        Add a new SSH key to your account for secure access to repositories.
                                    </DialogDescription>
                                </DialogHeader>
                                <div className="space-y-4 max-w-4/5">
                                    <div className="space-y-2">
                                        <Label htmlFor="keyTitle">Title</Label>
                                        <Input
                                            id="keyTitle"
                                            value={newKey.title}
                                            onChange={(e) => setNewKey((prev) => ({ ...prev, title: e.target.value }))}
                                            placeholder="e.g., MacBook Pro - Work"
                                        />
                                        <p className="text-xs text-muted-foreground">Choose a descriptive name to identify this key</p>
                                    </div>
                                    <div className="space-y-2">
                                        <Label htmlFor="keyContent">SSH Key</Label>
                                        <Textarea
                                            id="keyContent"
                                            value={newKey.key}
                                            onChange={(e) => setNewKey((prev) => ({ ...prev, key: e.target.value }))}
                                            placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG4rT3vTt99Ox5kndS4HmgTrKBT8SKzhK4rhGkEVGlue user@example.com"
                                            rows={4}
                                            className="font-mono text-sm"
                                        />
                                        <p className="text-xs text-muted-foreground">
                                            Paste your public SSH key here. It should start with ssh-rsa, ssh-ed25519, or ssh-ecdsa.
                                        </p>
                                    </div>
                                </div>
                                <DialogFooter style={{
                                    display: "flex",
                                    justifyContent: "end",
                                    paddingRight: "5rem"
                                }}>
                                    <Button variant="outline" onClick={() => setIsAddDialogOpen(false)}>
                                        Cancel
                                    </Button>
                                    <Button onClick={handleAddKey} disabled={isLoading}>
                                        {isLoading ? "Adding..." : "Add SSH Key"}
                                    </Button>
                                </DialogFooter>
                            </DialogContent>
                        </Dialog>
                    </div>

                    {!sshKeys || sshKeys.length === 0 ? (
                        <Card>
                            <CardContent className="flex flex-col items-center justify-center py-12">
                                <Key className="h-12 w-12 text-muted-foreground mb-4" />
                                <h3 className="text-lg font-semibold mb-2">No SSH Keys</h3>
                                <p className="text-muted-foreground text-center mb-4">
                                    You haven't added any SSH keys yet. Add one to enable secure access to your repositories.
                                </p>
                                <Button onClick={() => setIsAddDialogOpen(true)}>
                                    <Plus className="h-4 w-4 mr-2" />
                                    Add Your First SSH Key
                                </Button>
                            </CardContent>
                        </Card>
                    ) : (
                        <div className="space-y-4">
                            {sshKeys.map((sshKey) => (
                                <Card key={sshKey.uid}>
                                    <CardHeader>
                                        <div className="flex items-start justify-between">
                                            <div className="space-y-2">
                                                <div className="flex items-center gap-2">
                                                    <Key className="h-4 w-4" />
                                                    <CardTitle className="text-lg">{sshKey.name}</CardTitle>
                                                </div>
                                                <CardDescription>Added on {new Date(sshKey.created_at).toLocaleDateString()}</CardDescription>
                                            </div>
                                            <AlertDialog>
                                                <AlertDialogTrigger asChild>
                                                    <Button
                                                        variant="outline"
                                                        size="sm"
                                                        className="text-destructive hover:text-destructive bg-transparent"
                                                    >
                                                        <Trash2 className="h-4 w-4" />
                                                    </Button>
                                                </AlertDialogTrigger>
                                                <AlertDialogContent>
                                                    <AlertDialogHeader>
                                                        <AlertDialogTitle>Delete SSH Key</AlertDialogTitle>
                                                        <AlertDialogDescription>
                                                            Are you sure you want to delete the SSH key "{sshKey.name}"? This action cannot be undone
                                                            and you will lose access to repositories using this key.
                                                        </AlertDialogDescription>
                                                    </AlertDialogHeader>
                                                    <AlertDialogFooter>
                                                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                        <AlertDialogAction
                                                            onClick={() => handleDeleteKey(sshKey.uid)}
                                                            className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                                                        >
                                                            Delete Key
                                                        </AlertDialogAction>
                                                    </AlertDialogFooter>
                                                </AlertDialogContent>
                                            </AlertDialog>
                                        </div>
                                    </CardHeader>
                                    <CardContent className="space-y-4">
                                        <div className="space-y-3">
                                            <div className="flex items-center justify-between">
                                                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                                                    <Shield className="h-4 w-4" />
                                                    <span>Fingerprint:</span>
                                                </div>
                                                <Button
                                                    variant="ghost"
                                                    size="sm"
                                                    onClick={() => copyToClipboard(sshKey.fingerprint)}
                                                    className="h-auto p-1"
                                                >
                                                    <Copy className="h-3 w-3" />
                                                </Button>
                                            </div>
                                            <code className="block text-xs bg-muted p-2 rounded font-mono break-all">{sshKey.fingerprint}</code>
                                        </div>

                                        <div className="flex items-center gap-6 text-sm text-muted-foreground">
                                            <div className="flex items-center gap-1">
                                                <Calendar className="h-4 w-4" />
                                                <span>Added {new Date(sshKey.created_at).toLocaleDateString()}</span>
                                            </div>
                                            <div className="flex items-center gap-1">
                                                <Clock className="h-4 w-4" />
                                                <span>
                        {sshKey.updated_at ? `Last used ${new Date(sshKey.updated_at).toLocaleDateString()}` : "Never used"}
                      </span>
                                            </div>
                                        </div>
                                    </CardContent>
                                </Card>
                            ))}
                        </div>
                    )}

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

                    {/* Help Section */}
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-base">Need Help?</CardTitle>
                        </CardHeader>
                        <CardContent className="space-y-2 text-sm text-muted-foreground">
                            <p>SSH keys provide a secure way to access your repositories without entering your password each time.</p>
                            <ul className="list-disc list-inside space-y-1 ml-4">
                                <li>
                                    Generate a new SSH key:{" "}
                                    <code className="bg-muted px-1 rounded">ssh-keygen -t ed25519 -C "your_email@example.com"</code>
                                </li>
                                <li>
                                    Copy your public key: <code className="bg-muted px-1 rounded">cat ~/.ssh/id_ed25519.pub</code>
                                </li>
                            </ul>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </>
    )
}