"use client"

import { Link } from 'react-router-dom'
import { ArrowLeft, Database, Upload, FileText, Image, Music, Video } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from '@/components/ui/select'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import { Checkbox } from '@/components/ui/checkbox'
import {License} from "@/data/License.tsx";

export default function CreateDataset() {
    const dataTypes = [
        {id: 'text', label: 'Text', icon: FileText, description: 'Natural language processing, documents'},
        {id: 'image', label: 'Image', icon: Image, description: 'Computer vision, photos, graphics'},
        {id: 'audio', label: 'Audio', icon: Music, description: 'Speech recognition, music analysis'},
        {id: 'video', label: 'Video', icon: Video, description: 'Video analysis, motion detection'},
    ]

    return (
        <div className="h-full bg-gray-50 py-8">
            <div className="max-w-3xl mx-auto px-4">
                <div className="mb-8">
                    <Link
                        to="/init"
                        className="inline-flex items-center text-gray-600 hover:text-gray-900 mb-4"
                    >
                        <ArrowLeft size={16} className="mr-2"/>
                        Back to options
                    </Link>
                    <div className="flex items-center space-x-3 mb-2">
                        <Database className="text-orange-600" size={24}/>
                        <h1 className="text-3xl font-bold text-gray-900">Create Dataset</h1>
                    </div>
                    <p className="text-gray-600">
                        Upload and manage datasets for machine learning and data analysis projects.
                    </p>
                </div>

                <div className="space-y-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>Dataset Information</CardTitle>
                            <CardDescription>
                                Basic details about your dataset
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-6">.
                            <div className="space-y-2">
                                <Label htmlFor="owner">Owner *</Label>
                                <Select>
                                    <SelectTrigger className="w-full">
                                        <SelectValue placeholder="Select a owner"/>
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
                                <Label htmlFor="dataset-name">Dataset name *</Label>
                                <Input
                                    id="dataset-name"
                                    placeholder="my-dataset"
                                    className="font-mono"
                                />
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="dataset-description">Description</Label>
                                <Textarea
                                    id="dataset-description"
                                    placeholder="Describe your dataset, its purpose, and contents..."
                                    rows={4}
                                />
                            </div>

                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div className="space-y-2">
                                    <Label>License</Label>
                                    <Select>
                                        <SelectTrigger>
                                            <SelectValue placeholder="Choose a license"/>
                                        </SelectTrigger>
                                        <SelectContent style={{
                                            border: "none",
                                        }}>
                                            {
                                                Object.keys(License).map((item, index) => (
                                                    <SelectItem value={item} key={item + index}>{item}</SelectItem>
                                                ))
                                            }
                                        </SelectContent>
                                    </Select>
                                </div>
                                <div className="space-y-2">
                                    <Label>Task category</Label>
                                    <Select>
                                        <SelectTrigger>
                                            <SelectValue placeholder="Select task type" />
                                        </SelectTrigger>
                                        <SelectContent style={{
                                            border: "none",
                                        }}>
                                            <SelectItem value="classification">Classification</SelectItem>
                                            <SelectItem value="regression">Regression</SelectItem>
                                            <SelectItem value="clustering">Clustering</SelectItem>
                                            <SelectItem value="nlp">Natural Language Processing</SelectItem>
                                            <SelectItem value="computer-vision">Computer Vision</SelectItem>
                                            <SelectItem value="other">Other</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>Data Type</CardTitle>
                            <CardDescription>
                                What type of data does your dataset contain?
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <RadioGroup defaultValue="text" className="space-y-3">
                                {dataTypes.map((type) => {
                                    const IconComponent = type.icon
                                    return (
                                        <div key={type.id} className="flex items-start space-x-3 p-4 border border-gray-200 rounded-lg">
                                            <RadioGroupItem value={type.id} id={type.id} className="mt-1" />
                                            <div className="flex-1">
                                                <div className="flex items-center space-x-2 mb-1">
                                                    <IconComponent size={16} className="text-orange-600" />
                                                    <Label htmlFor={type.id} className="font-medium">{type.label}</Label>
                                                </div>
                                                <p className="text-sm text-gray-600">
                                                    {type.description}
                                                </p>
                                            </div>
                                        </div>
                                    )
                                })}
                            </RadioGroup>
                        </CardContent>
                    </Card>

                    <Card>
                        <CardHeader>
                            <CardTitle>Upload Data</CardTitle>
                            <CardDescription>
                                Upload your dataset files
                            </CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-gray-400 transition-colors">
                                <Upload className="mx-auto h-12 w-12 text-gray-400 mb-4" />
                                <p className="text-lg font-medium text-gray-900 mb-2">
                                    Drop files here or click to upload
                                </p>
                                <p className="text-sm text-gray-500 mb-4">
                                    Supports CSV, JSON, Parquet, and compressed files
                                </p>
                                <Button variant="outline">
                                    Choose Files
                                </Button>
                            </div>

                            <div className="space-y-3">
                                <div className="flex items-center space-x-2">
                                    <Checkbox id="auto-split" />
                                    <Label htmlFor="auto-split" className="text-sm">
                                        Automatically split into train/validation/test sets
                                    </Label>
                                </div>
                                <div className="flex items-center space-x-2">
                                    <Checkbox id="generate-preview" defaultChecked />
                                    <Label htmlFor="generate-preview" className="text-sm">
                                        Generate data preview and statistics
                                    </Label>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <div className="flex space-x-3">
                        <Button className="flex-1">
                            Create Dataset
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
