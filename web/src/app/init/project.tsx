"use client"

import { Link } from 'react-router-dom'
import { ArrowLeft, FolderOpen, Target, Users } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from '@/components/ui/select'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'

export default function CreateProject() {
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
                        <FolderOpen className="text-purple-600" size={24} />
                        <h1 className="text-3xl font-bold text-gray-900">Create Project</h1>
                    </div>
                    <p className="text-gray-600">
                        Start a new project to organize your work and track progress effectively.
                    </p>
                </div>

                <div className="space-y-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Project Details</CardTitle>
                            <CardDescription>
                                Basic information about your project
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">
                            <div className="space-y-2">
                                <Label htmlFor="owner">Owner *</Label>
                                <Select>
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Select a owner" />
                                    </SelectTrigger>
                                    <SelectContent style={{
                                        border: "none",
                                    }}>
                                        <SelectGroup>
                                            <SelectItem value="owner">Owner</SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                            </div>
                            <div className="space-y-2">
                                <Label htmlFor="project-name">Project name *</Label>
                                <Input
                                    id="project-name"
                                    placeholder="My Awesome Project"
                                />
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="project-description">Description</Label>
                                <Textarea
                                    id="project-description"
                                    placeholder="Describe what this project is about..."
                                    rows={4}
                                />
                            </div>

                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div className="space-y-2">
                                    <Label htmlFor="start-date">Start date</Label>
                                    <Input
                                        id="start-date"
                                        type="date"
                                    />
                                </div>
                                <div className="space-y-2">
                                    <Label htmlFor="end-date">Target end date</Label>
                                    <Input
                                        id="end-date"
                                        type="date"
                                    />
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>Project Settings</CardTitle>
                            <CardDescription>
                                Configure how your project will be managed
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">
                            <div className="space-y-2">
                                <Label>Project template</Label>
                                <Select>
                                    <SelectTrigger>
                                        <SelectValue placeholder="Choose a template" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="blank">Blank project</SelectItem>
                                        <SelectItem value="kanban">Kanban board</SelectItem>
                                        <SelectItem value="scrum">Scrum project</SelectItem>
                                        <SelectItem value="bug-tracking">Bug tracking</SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>

                            <div className="space-y-4">
                                <Label>Visibility</Label>
                                <RadioGroup defaultValue="team" className="space-y-3">
                                    <div className="flex items-start space-x-3 p-4 border rounded-lg">
                                        <RadioGroupItem value="private" id="project-private" className="mt-1" />
                                        <div className="flex-1">
                                            <div className="flex items-center space-x-2 mb-1">
                                                <Target size={16} className="text-red-600" />
                                                <Label htmlFor="project-private" className="font-medium">Private</Label>
                                            </div>
                                            <p className="text-sm text-gray-600">
                                                Only you can access this project.
                                            </p>
                                        </div>
                                    </div>
                                    <div className="flex items-start space-x-3 p-4 border rounded-lg">
                                        <RadioGroupItem value="team" id="project-team" className="mt-1" />
                                        <div className="flex-1">
                                            <div className="flex items-center space-x-2 mb-1">
                                                <Users size={16} className="text-blue-600" />
                                                <Label htmlFor="project-team" className="font-medium">Team</Label>
                                            </div>
                                            <p className="text-sm text-gray-600">
                                                All team members can access this project.
                                            </p>
                                        </div>
                                    </div>
                                </RadioGroup>
                            </div>
                        </CardContent>
                    </Card>

                    <div className="flex space-x-3">
                        <Button className="flex-1">
                            Create Project
                        </Button>
                        <Button variant="outline" asChild>
                            <Link to="/init">Cancel</Link>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    )
}
