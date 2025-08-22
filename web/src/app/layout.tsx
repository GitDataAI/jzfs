import {AppSidebar} from "@/components/shell/app-sidebar.tsx";
import {
    SidebarInset,
    SidebarProvider,
} from '@/components/ui/sidebar'
import {Outlet} from "react-router-dom";
import {DefaultNavFeedback, DefaultNavMain} from "@/data/system/homeNav.tsx";

export const AppLayout = () => {
    return(
        <>
            <SidebarProvider>
                <AppSidebar main={DefaultNavMain} feedback={DefaultNavFeedback}/>
                <SidebarInset>
                    <Outlet/>
                </SidebarInset>
            </SidebarProvider>
        </>
    )
}