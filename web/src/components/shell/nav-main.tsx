"use client"

import {ChevronRight} from "lucide-react"

import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from '@/components/ui/collapsible'
import {
    SidebarGroup,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuSub,
    SidebarMenuSubButton,
    SidebarMenuSubItem,
} from '@/components/ui/sidebar'
import type {NavItem} from "@/hooks/use-nav-data.tsx";
import {useNavigate} from "react-router-dom";

export function NavMain({props}: { props: NavItem }) {
    const items = props.items;
    const title = props.title;
    const nav = useNavigate();
    const url = window.location.pathname;

    return (
        <SidebarGroup>
            <SidebarGroupLabel>{title}</SidebarGroupLabel>
            <SidebarMenu>
                {items.map((item) => (
                    <Collapsible
                        key={item.title}
                        asChild
                        defaultOpen={item.isActive}
                    >
                        <div>
                            {
                                item.items ? (
                                    <>
                                        <CollapsibleTrigger asChild>
                                            <SidebarMenuButton tooltip={item.title}>
                                                {item.icon && <item.icon />}
                                                <span>{item.title}</span>
                                                <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
                                            </SidebarMenuButton>
                                        </CollapsibleTrigger>
                                        <CollapsibleContent>
                                            <SidebarMenuSub>
                                                {item.items?.map((subItem) => (
                                                    <SidebarMenuSubItem key={subItem.title} style={{
                                                        backgroundColor: url.startsWith(item.url) ? "#e8e8e8" : "transparent",
                                                        borderRadius: "4px",
                                                    }}>
                                                        <SidebarMenuSubButton asChild>
                                                            <a onClick={()=>nav(subItem.url)}>
                                                                <span>{subItem.title}</span>
                                                            </a>
                                                        </SidebarMenuSubButton>
                                                    </SidebarMenuSubItem>
                                                ))}
                                            </SidebarMenuSub>
                                        </CollapsibleContent>
                                    </>
                                ):(
                                    <SidebarMenuSubItem key={item.title} className="cursor-pointer" id={item.title} style={{
                                        backgroundColor: url.startsWith(item.url) ? "#e8e8e8" : "transparent",
                                        borderRadius: "4px",
                                    }}>
                                        <SidebarMenuSubButton asChild>
                                            <a onClick={()=>{
                                                nav(item.url)
                                            }}>
                                                {item.icon && <item.icon />}
                                                <span>{item.title}</span>
                                            </a>
                                        </SidebarMenuSubButton>
                                    </SidebarMenuSubItem>
                                )
                            }
                        </div>
                    </Collapsible>
                ))}
            </SidebarMenu>
        </SidebarGroup>
    )
}
