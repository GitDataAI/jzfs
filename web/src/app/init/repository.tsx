"use client"

import {Link, useNavigate} from 'react-router-dom'
import { ArrowLeft, GitBranch, Lock, Globe } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import {
    Select,
    SelectContent, SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"
import {useEffect, useState} from "react";
import {type RepoInitBefore, type RepoInitStorage, type RepoOwnerSelectItem, useInit} from "@/hooks/use-init.tsx";
import {toast} from "sonner";
export default function CreateRepository() {
    const [select, setSelect] = useState<RepoOwnerSelectItem[] | null>([]);
    const [repo_name, setRepoName] = useState<string>("");
    const [repo_description, setRepoDescription] = useState<string>("");
    const [repo_is_private, setRepoIsPrivate] = useState<boolean>(false);
    const [repo_default_branch, setRepoDefaultBranch] = useState<string>("main");
    const [repo_owner, set_repo_owner] = useState<RepoOwnerSelectItem | null>(null)
    const init =  useInit();
    const [storage, setStorage] = useState<RepoInitStorage[]>([]);
    const [select_storage, set_select_storage] = useState<RepoInitStorage | null>(null);
    useEffect(() => {
        init.InitRepoOwnerSelect()
            .then(res=>{
                setSelect(res);
            })
            .catch(err=>{
                toast.error(err)
            })
        init.InitRepoStorage()
            .then(res=>{
                if (typeof res !== "string") {
                    setStorage(res);
                }
            })
            .catch(err=>{
                toast.error(err)
            })
    }, []);
    const [repo_name_error, set_repo_name_error] = useState("");
    const handleSubmitBefore = async () => {
        if (repo_owner) {
            if (repo_name !== "") {
                let before: RepoInitBefore = {
                    owner_uid: repo_owner.uid,
                    team: repo_owner.team,
                    repo_name: repo_name,
                }
                const res = await init.InitRepoBefore(before);
                if (res !== "") {
                    toast.error(res);
                    set_repo_name_error(res)
                } else {
                    return true;
                }
            } else {
                toast.error("Please input repository name")
            }
        } else {
            toast.error("Please select a owner")
        }
        return false;
    };

    const nav = useNavigate();
    const handleSubmit = async () => {
        if (!await handleSubmitBefore()) {
            return;
        }
        if (select_storage === null) {
            toast.error("Please select a storage")
            return;
        }
        if (repo_owner) {
            const payload = {
                owner_uid: repo_owner.uid,
                team: repo_owner.team,
                repo_name: repo_name,
                repo_description: repo_description,
                repo_is_private: repo_is_private,
                repo_default_branch: repo_default_branch,
            };
            init.InitRepo(payload)
                .then(res=>{
                    if (res === "") {
                        toast.success("Create repository success")
                        nav(`/${repo_owner.username}/${repo_name}`)
                    } else {
                        console.log(res);
                        toast.error("Create repository failed")
                    }
                })
                .catch(err=>{
                    toast.error(err)
                })
        }
    };
    return (
        <div className="h-full bg-gray-50 py-8">
            <div className="max-w-3xl mx-auto px-4">
                <div className="mb-8">
                    <Link
                        to="/init"
                        className="inline-flex items-center text-gray-600 hover:text-gray-900 mb-4"
                    >
                        <ArrowLeft size={16} className="mr-2" />
                        Back to options
                    </Link>
                    <div className="flex items-center space-x-3 mb-2">
                        <GitBranch className="text-blue-600" size={24} />
                        <h1 className="text-3xl font-bold text-gray-900">Create Repository</h1>
                    </div>
                    <p className="text-gray-600">
                        Create a new repository to store your project files and collaborate with others.
                    </p>
                </div>

                <Card>
                    <CardHeader>
                        <CardTitle>Repository Details</CardTitle>
                        <CardDescription>
                            Choose a name、owner and description for your new repository
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-6">
                        <div className="space-y-2">
                            <Label htmlFor="owner">Owner *</Label>
                            <Select
                                required
                                defaultValue={select?.find(item=>!item.team)?.uid || ""}
                                onValueChange={(value) => {
                                    const item = select?.find(item=>item.uid === value);
                                    if (item) {
                                        set_repo_owner(item)
                                    }
                                }}
                            >
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="Select a owner" />
                                </SelectTrigger>
                                <SelectContent
                                    style={{
                                    border: "none",
                                }}>
                                    <SelectGroup>
                                        {
                                            select && select.map((item, index) => (
                                                <SelectItem key={index} value={item.uid}>
                                                    {item.username}
                                                </SelectItem>
                                            ))
                                        }
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                        <div className="space-y-2">
                            <Label htmlFor="repo-name">Repository name *</Label>
                            <Input
                                id="repo-name"
                                placeholder="my-awesome-project"
                                className="font-mono"
                                value={repo_name}
                                onChange={(e) => setRepoName(e.target.value)}
                            />
                            <p className="text-sm text-gray-500">
                                Great repository names are short and memorable.
                            </p>
                            {
                                repo_name_error && (
                                    <p className="text-sm text-red-500">{repo_name_error}</p>
                                )
                            }
                        </div>
                        <div className="space-y-2">
                            <Label htmlFor="description">Description (optional)</Label>
                            <Textarea
                                value={repo_description}
                                onChange={(e) => setRepoDescription(e.target.value)}
                                id="description"
                                placeholder="A brief description of your project"
                                rows={3}
                            />
                        </div>

                        <div className="space-y-4">
                            <Label>Visibility</Label>
                            <RadioGroup defaultValue="public" className="space-y-3">
                                <div className="flex items-start space-x-3 p-4 border  border-gray-200 rounded-lg">
                                    <RadioGroupItem value="public" id="public" className="mt-1" onClick={() => setRepoIsPrivate(false)} />
                                    <div className="flex-1">
                                        <div className="flex items-center space-x-2 mb-1">
                                            <Globe size={16} className="text-green-600" />
                                            <Label htmlFor="public" className="font-medium">Public</Label>
                                        </div>
                                        <p className="text-sm text-gray-600">
                                            Anyone on the internet can see this repository. You choose who can commit.
                                        </p>
                                    </div>
                                </div>
                                <div className="flex items-start space-x-3 p-4 border border-gray-200 rounded-lg">
                                    <RadioGroupItem value="private" id="private" className="mt-1" onClick={() => setRepoIsPrivate(true)} />
                                    <div className="flex-1">
                                        <div className="flex items-center space-x-2 mb-1">
                                            <Lock size={16} className="text-orange-600" />
                                            <Label htmlFor="private" className="font-medium">Private</Label>
                                        </div>
                                        <p className="text-sm text-gray-600">
                                            You choose who can see and commit to this repository.
                                        </p>
                                    </div>
                                </div>
                            </RadioGroup>
                        </div>
                        <div className="space-y-2">
                            <Label>Default Branches</Label>
                            <Input
                                id="repo-default-branch"
                                placeholder="main"
                                className="font-mono"
                                value={repo_default_branch}
                                onChange={(e) => setRepoDefaultBranch(e.target.value)}
                            />
                            <p className="text-sm text-gray-500">
                                The default branch for your new repository.
                            </p>
                        </div>
                        <div className="space-y-2">
                            <Label htmlFor="storage">Storage *</Label>
                            <Select
                                required
                                onValueChange={(value) => {
                                    const item:RepoInitStorage = JSON.parse(value);
                                    if (item) {
                                        set_select_storage(item)
                                    }
                                }}
                            >
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="Select a storage node" />
                                </SelectTrigger>
                                <SelectContent
                                    style={{
                                        border: "none",
                                    }}>
                                    <SelectGroup>
                                        {
                                            storage && storage.map((item, index) => (
                                                <SelectItem key={index} value={JSON.stringify(item)}>
                                                    {item.name}
                                                </SelectItem>
                                            ))
                                        }
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>

                        <div className="flex space-x-3 pt-4">
                            <Button className="flex-1"  type="button" onClick={handleSubmit}>
                                Create Repository
                            </Button>
                            <Button variant="outline" asChild>
                                <Link to="/init">Cancel</Link>
                            </Button>
                        </div>
                    </CardContent>
                </Card>
            </div>
        </div>
    )
}
