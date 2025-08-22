"use client"

import { Link } from 'react-router-dom'
import { GitBranch, Users, FolderOpen, Database, ArrowRight } from 'lucide-react'

export default function InitPage() {
    const options = [
        {
            title: 'Create Repository',
            description: 'Create a new repository to store your code and collaborate with others',
            icon: GitBranch,
            path: '/init/repository',
            color: 'bg-blue-50 border-blue-200 hover:bg-blue-100',
            iconColor: 'text-blue-600'
        },
        {
            title: 'Create Team',
            description: 'Set up a team to collaborate on projects and manage permissions',
            icon: Users,
            path: '/init/team',
            color: 'bg-green-50 border-green-200 hover:bg-green-100',
            iconColor: 'text-green-600'
        },
        {
            title: 'Create Project',
            description: 'Start a new project to organize your work and track progress',
            icon: FolderOpen,
            path: '/init/project',
            color: 'bg-purple-50 border-purple-200 hover:bg-purple-100',
            iconColor: 'text-purple-600'
        },
        {
            title: 'Create Dataset',
            description: 'Upload and manage datasets for machine learning and data analysis',
            icon: Database,
            path: '/init/dataset',
            color: 'bg-orange-50 border-orange-200 hover:bg-orange-100',
            iconColor: 'text-orange-600'
        }
    ]

    return (
        <div className="h-full flex items-center justify-center p-4">
            <div className="w-full max-w-4xl">
                <div className="text-center mb-12">
                    <h1 className="text-4xl font-bold text-gray-900 mb-4">
                        Get Started
                    </h1>
                    <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                        Choose what you'd like to create to begin your journey. Each option will guide you through the setup process.
                    </p>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {options.map((option) => {
                        const IconComponent = option.icon
                        return (
                            <Link
                                key={option.path}
                                to={option.path}
                                className={`block p-8 rounded-xl border-2 transition-all duration-200 ${option.color} group`}
                            >
                                <div className="flex items-start space-x-4">
                                    <div className={`p-3 rounded-lg bg-white shadow-sm ${option.iconColor}`}>
                                        <IconComponent size={24} />
                                    </div>
                                    <div className="flex-1">
                                        <h3 className="text-xl font-semibold text-gray-900 mb-2 group-hover:text-gray-700">
                                            {option.title}
                                        </h3>
                                        <p className="text-gray-600 mb-4 leading-relaxed">
                                            {option.description}
                                        </p>
                                        <div className="flex items-center text-sm font-medium text-gray-500 group-hover:text-gray-700">
                                            Get started
                                            <ArrowRight size={16} className="ml-2 transition-transform group-hover:translate-x-1" />
                                        </div>
                                    </div>
                                </div>
                            </Link>
                        )
                    })}
                </div>

                <div className="mt-12 text-center">
                    <p className="text-gray-500">
                        Need help getting started? Check out our{' '}
                        <a href="#" className="text-blue-600 hover:text-blue-700 font-medium">
                            documentation
                        </a>{' '}
                        or{' '}
                        <a href="#" className="text-blue-600 hover:text-blue-700 font-medium">
                            contact support
                        </a>
                    </p>
                </div>
            </div>
        </div>
    )
}


