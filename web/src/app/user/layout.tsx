import {useEffect, useState} from "react"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { Users, BookOpen, Star, MapPin, LinkIcon, Calendar, Building, Mail } from "lucide-react"
import { Suspense } from "react"
import {SidebarInset, SidebarProvider} from "@/components/ui/sidebar.tsx";
import {AppSidebar} from "@/components/shell/app-sidebar.tsx";
import {Outlet, useNavigate, useParams} from "react-router-dom";
import {DefaultUserNavItem} from "@/data/system/homeNav.tsx";
import useUserData, {type UserData, UserDataContext} from "@/hooks/use-user-data.tsx";
import {formatRelativeTime} from "@/lib/utils.ts";


export const UsersLayout = () => {
    const { username } = useParams();
    const  data = useUserData();
    const [userData, setUserData] = useState<UserData | null>(null);
    const nav = useNavigate();
    useEffect(() => {
        if (username) {
            data.fetchUserData(username).then(res=>{
                if (res) {
                    setUserData(res);
                }
            })
        }
    }, []);
    const userStats = {
        followers: 0,
        following: 0,
        repositories: 0,
        stars: 0,
    }

    return(
        <>
            {
                username && userData  && (
                    <SidebarProvider>
                        <AppSidebar main={DefaultUserNavItem(username)}/>
                        <SidebarInset>
                            <Suspense fallback={<div>Loading...</div>}>
                                <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 top-0 z-50">
                                    <div className="max-w-7xl mx-auto px-4 py-6">
                                        <div className="flex flex-col md:flex-row items-center gap-6">
                                            <Avatar className="w-24 h-24">
                                                <AvatarImage src="/professional-developer-avatar.png" alt="User Avatar" />
                                                <AvatarFallback className="text-xl">JD</AvatarFallback>
                                            </Avatar>

                                            <div className="flex-1 text-center md:text-left space-y-3">
                                                <div>
                                                    <h1 className="text-2xl font-bold text-foreground">{userData.model.display_name}</h1>
                                                    <p className="text-lg text-muted-foreground">@{username}</p>
                                                </div>

                                                <p className="text-foreground max-w-2xl">
                                                    {userData.model.bio}
                                                </p>

                                                <div className="flex flex-wrap justify-center md:justify-start gap-4 text-sm text-muted-foreground">
                                                    {
                                                        userData.model.company && (
                                                            <div className="flex items-center gap-1">
                                                                <Building className="w-4 h-4" />
                                                                <span>{userData.model.company}</span>
                                                            </div>
                                                        )
                                                    }
                                                    {
                                                        userData.model.location && (
                                                            <div className="flex items-center gap-1">
                                                                <MapPin className="w-4 h-4" />
                                                                <span>{userData.model.location}</span>
                                                            </div>
                                                        )
                                                    }
                                                    {
                                                        userData.model.website_url && (
                                                            <div className="flex items-center gap-1">
                                                                <LinkIcon className="w-4 h-4" />
                                                                <span>{userData.model.website_url}</span>
                                                            </div>
                                                        )
                                                    }
                                                    {
                                                        userData.model.email && (
                                                            <div className="flex items-center gap-1">
                                                                <Mail className="w-4 h-4" />
                                                                <span>{userData.model.email}</span>
                                                            </div>
                                                        )
                                                    }
                                                    <div className="flex items-center gap-1">
                                                        <Calendar className="w-4 h-4" />
                                                        <span>Joined {formatRelativeTime((new Date(userData.model.created_at).getTime()))}</span>
                                                    </div>
                                                </div>

                                                <div className="flex justify-center md:justify-start gap-6 text-center">
                                                    <div className="flex flex-col">
                                                        <div className="flex items-center gap-1">
                                                            <Users className="w-4 h-4" />
                                                            <span className="font-semibold">{userStats.followers.toLocaleString()}</span>
                                                        </div>
                                                        <span className="text-sm text-muted-foreground">followers</span>
                                                    </div>
                                                    <div className="flex flex-col">
                                                        <div className="flex items-center gap-1">
                                                            <Users className="w-4 h-4" />
                                                            <span className="font-semibold">{userStats.following.toLocaleString()}</span>
                                                        </div>
                                                        <span className="text-sm text-muted-foreground">following</span>
                                                    </div>
                                                    <div className="flex flex-col">
                                                        <div className="flex items-center gap-1">
                                                            <BookOpen className="w-4 h-4" />
                                                            <span className="font-semibold">{userStats.repositories}</span>
                                                        </div>
                                                        <span className="text-sm text-muted-foreground">repositories</span>
                                                    </div>
                                                    <div className="flex flex-col">
                                                        <div className="flex items-center gap-1">
                                                            <Star className="w-4 h-4" />
                                                            <span className="font-semibold">{userStats.stars.toLocaleString()}</span>
                                                        </div>
                                                        <span className="text-sm text-muted-foreground">stars</span>
                                                    </div>
                                                </div>

                                                <div className="flex justify-center md:justify-start gap-2">
                                                    {
                                                        userData.is_owner ? (
                                                            <Button onClick={()=>{
                                                                nav("/setting/profile")
                                                            }}>
                                                                Edit
                                                            </Button>
                                                        ):(
                                                            <>
                                                                {
                                                                    userData.is_follow ? (
                                                                        <Button variant="outline" onClick={() => {
                                                                        }}>Unfollow</Button>
                                                                    ):(
                                                                        <Button onClick={() => {
                                                                        }}>Follow</Button>
                                                                    )
                                                                }
                                                            </>
                                                        )
                                                    }
                                                    {
                                                        !userData.is_owner && (
                                                            <Button variant="outline">Message</Button>
                                                        )
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </Suspense>
                            <div className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
                               <div className="max-w-7xl mx-auto px-4 py-6">
                                 <UserDataContext value={userData}>
                                     <Outlet />
                                 </UserDataContext>
                               </div>
                            </div>
                        </SidebarInset>
                    </SidebarProvider>
                )
            }
        </>
    )
}