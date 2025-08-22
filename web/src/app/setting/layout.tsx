import {SidebarInset, SidebarProvider} from "@/components/ui/sidebar.tsx";
import {AppSidebar} from "@/components/shell/app-sidebar.tsx";
import {DefaultUserSetting} from "@/data/system/homeNav.tsx";
import {Outlet} from "react-router-dom";

export const UserSettingLayout = () => {
    return(
        <>
            <SidebarProvider>
                <AppSidebar main={DefaultUserSetting}/>
                <SidebarInset>
                    <Outlet/>
                </SidebarInset>
            </SidebarProvider>
        </>
    )
}