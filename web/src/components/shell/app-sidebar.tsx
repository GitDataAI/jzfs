"use client"
import {
    House,
    type LucideIcon, Mail,
    Plus, User,
} from "lucide-react"


import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
} from '@/components/ui/sidebar'
import {NavUser} from "@/components/shell/nav-user.tsx";
import {NavMain} from "@/components/shell/nav-main.tsx";
// import {NavProjects} from "@/components/shell/nav-project.tsx";
import {Button} from "@/components/ui/button.tsx";
import { useAuths } from "@/hooks/use-auths.tsx";

import type {NavItem} from "@/hooks/use-nav-data.tsx";
import {useNavigate} from "react-router-dom";


export interface AppSidebarProps {
    main?: NavItem;
    feedback?: NavItem;
    projects?: {
        name: string
        url: string
        icon: LucideIcon
    },
}

export function AppSidebar(props: AppSidebarProps) {
    const user = useAuths();
    const nav = useNavigate();
    return (
        <Sidebar collapsible="icon" style={{
            border: "none"
        }}>
            <SidebarHeader>
                <div className="w-full p-4 border border-gray-200 bg-sidebar rounded-lg group-data-[collapsible=icon]:overflow-hidden">
                    <div className="grid grid-cols-3 gap-3">
                        <Button className="bg-white text-black hover:bg-gray-200" onClick={()=>{
                            nav("/")
                        }}>
                            <House />
                        </Button>
                        {
                            user.session ? (
                                <>
                                    <Button className="bg-white text-black hover:bg-gray-200" onClick={()=>{
                                        nav("/" + user.session?.username)
                                    }}>
                                        <User />
                                    </Button>

                                    <Button className="bg-white text-black hover:bg-gray-200" onClick={()=>{
                                        nav("/init")
                                    }}>
                                        <Plus />
                                    </Button>
                                    <Button className="bg-white text-black hover:bg-gray-200">
                                        <Mail />
                                    </Button>
                                </>
                            ):(
                                <>
                                    <Button className="bg-white text-black hover:bg-gray-200" onClick={()=>{
                                        nav("/auth/login")
                                    }}>
                                        <User />
                                    </Button>
                                </>
                            )
                        }


                    </div>
                </div>
            </SidebarHeader>
            <SidebarContent>
                {
                    props.main && (
                        <NavMain props={props.main} />
                    )
                }
                {
                    props.feedback && (
                        <NavMain props={props.feedback} />
                    )
                }
            </SidebarContent>
            <SidebarFooter>
                {
                    user.isAuthenticated && user.session && (
                        <NavUser user={{
                            name: user.session.username,
                            email: user.session.email,
                            avatar: user.session.avatar_url,
                        }} />
                    )
                }
            </SidebarFooter>
        </Sidebar>
    )
}
