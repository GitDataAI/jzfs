"use client"

import { Link } from 'react-router-dom'
import { ArrowLeft, Users, Mail, Plus, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { useState } from 'react'

export default function CreateTeam() {
    const [members, setMembers] = useState<string[]>([])
    const [newMember, setNewMember] = useState('')

    const addMember = () => {
        if (newMember.trim() && !members.includes(newMember.trim())) {
            setMembers([...members, newMember.trim()])
            setNewMember('')
        }
    }

    const removeMember = (member: string) => {
        setMembers(members.filter(m => m !== member))
    }

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
                        <Users className="text-green-600" size={24} />
                        <h1 className="text-3xl font-bold text-gray-900">Create Team</h1>
                    </div>
                    <p className="text-gray-600">
                        Set up a team to collaborate on projects and manage permissions together.
                    </p>
                </div>

                <div className="space-y-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Team Information</CardTitle>
                            <CardDescription>
                                Basic information about your team
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">
                            <div className="space-y-2">
                                <Label htmlFor="team-name">Team name *</Label>
                                <Input
                                    id="team-name"
                                    placeholder="awesome-team"
                                    className="font-mono"
                                />
                                <p className="text-sm text-gray-500">
                                    Choose a unique name for your team.
                                </p>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="team-description">Description (optional)</Label>
                                <Textarea
                                    id="team-description"
                                    placeholder="What does your team work on?"
                                    rows={3}
                                />
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>Team Members</CardTitle>
                            <CardDescription>
                                Invite people to join your team
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="flex space-x-2">
                                <div className="flex-1">
                                    <Input
                                        placeholder="Enter email address"
                                        value={newMember}
                                        onChange={(e) => setNewMember(e.target.value)}
                                        onKeyPress={(e) => e.key === 'Enter' && addMember()}
                                    />
                                </div>
                                <Button onClick={addMember} size="icon">
                                    <Plus size={16} />
                                </Button>
                            </div>

                            {members.length > 0 && (
                                <div className="space-y-2">
                                    <Label>Invited members:</Label>
                                    <div className="flex flex-wrap gap-2">
                                        {members.map((member) => (
                                            <Badge key={member} variant="secondary" className="flex items-center space-x-1">
                                                <Mail size={12} />
                                                <span>{member}</span>
                                                <button
                                                    onClick={() => removeMember(member)}
                                                    className="ml-1 hover:text-red-600"
                                                >
                                                    <X size={12} />
                                                </button>
                                            </Badge>
                                        ))}
                                    </div>
                                </div>
                            )}

                            <p className="text-sm text-gray-500">
                                Team members will receive an invitation email to join your team.
                            </p>
                        </CardContent>
                    </Card>

                    <div className="flex space-x-3">
                        <Button className="flex-1">
                            Create Team
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
