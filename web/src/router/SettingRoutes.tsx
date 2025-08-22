import type {RouteObject} from "react-router-dom";
import {UserSettingLayout} from "@/app/setting/layout.tsx";
import {UserSettingProfilePage} from "@/app/setting/profile.tsx";
import {UserSettingAccountPage} from "@/app/setting/account.tsx";
import {UserSettingSecurityPage} from "@/app/setting/security.tsx";
import {UserSettingNotifyPage} from "@/app/setting/notify.tsx";
import {UserSettingPreferencePage} from "@/app/setting/preference.tsx";
import {UserSettingBillingPage} from "@/app/setting/biling.tsx";
import {UserSettingSSHKeyPage} from "@/app/setting/ssh-key.tsx";
import {UserSettingAccessKeyPage} from "@/app/setting/access-key.tsx";

export const UserSettingRoutes:RouteObject[] = [
    {
        path: "/setting",
        element: <UserSettingLayout/>,
        children: [
            {
                path: "profile",
                element: <UserSettingProfilePage/>
            },
            {
                path: "account",
                element: <UserSettingAccountPage/>
            },
            {
                path: "security",
                element: <UserSettingSecurityPage/>
            },
            {
                path: "notifications",
                element: <UserSettingNotifyPage/>
            },
            {
                path: "preferences",
                element: <UserSettingPreferencePage/>
            },
            {
                path: "billing",
                element: <UserSettingBillingPage/>
            },
            {
                path: "ssh",
                element: <UserSettingSSHKeyPage/>
            },
            {
                path: "access-key",
                element: <UserSettingAccessKeyPage/>
            }
        ]
    }
]