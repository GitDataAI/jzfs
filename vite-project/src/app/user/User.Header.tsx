import {Tab, Tabs} from "@heroui/tabs";
import {useSearchParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {VscLayersActive} from "react-icons/vsc";
import {IconWrapper} from "@/app/repo/Repo.Icons.tsx";
import {RiGitRepositoryLine, RiUserFollowLine} from "react-icons/ri";
import {GoPackage} from "react-icons/go";
import {MdProductionQuantityLimits} from "react-icons/md";
import {CiSettings, CiStar} from "react-icons/ci";
import useUser from "@/state/useUser.tsx";
import {UserDashBored} from "@/types.ts";

export const UserHeader = (props: { setTab: (arg0: string) => void, user?: UserDashBored }) => {
    const [Query , setQuery] = useSearchParams();
    const [ Tabes, setTab ] = useState("active");
    const user = useUser();
    // useEffect(()=>{
    //    if (!Query.get("tab")){
    //        Query.set("tab","active")
    //        setQuery(Query)
    //        setTab("active")
    //        props.setTab("active")
    //    } else {
    //        setTab(Query.get("tab") as string)
    //        props.setTab(Query.get("tab") as string)
    //    }
    // },[Query,props,setQuery])
    useEffect(()=>{
        setTab(Query.get("tab") as string)
        props.setTab(Query.get("tab") as string)
    },[Query, Tabes, props, setQuery])
    return (
        <div className="user-header">
            <Tabs variant="bordered" className="user-header-tabs" onSelectionChange={(x)=>{
                setTab(x.toString());
                Query.set("tab",x.toString())
                setQuery(Query)
                props.setTab(x.toString())
            }} selectedKey={Tabes}>
                <Tab key="active" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <VscLayersActive />
                            <span className="ml-2">Active</span>
                        </IconWrapper>
                    </div>
                }/>
                <Tab key="reposiotry" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <RiGitRepositoryLine />
                            <span className="ml-2">Reposiotry</span>
                        </IconWrapper>
                    </div>
                }/>
                <Tab key="package" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <GoPackage />
                            <span className="ml-2">Package</span>
                        </IconWrapper>
                    </div>
                }/>
                <Tab key="product" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <MdProductionQuantityLimits />
                            <span className="ml-2">Product</span>
                        </IconWrapper>
                    </div>
                }/>
                <Tab key="star" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <CiStar />
                            <span className="ml-2">Star</span>
                        </IconWrapper>
                    </div>
                }/>
                <Tab key="follow" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <RiUserFollowLine />
                            <span className="ml-2">Follow</span>
                        </IconWrapper>
                    </div>
                }/>
                {
                    (user.user && props.user) && (
                        <>
                            {
                                (user.user.uid === props.user.user.uid) && (
                                    <Tab key="setting" title={
                                        <div className="flex items-center">
                                            <IconWrapper className="  text-black">
                                                <CiSettings />
                                                <span className="ml-2">Setting</span>
                                            </IconWrapper>
                                        </div>
                                    }/>
                                )
                            }
                        </>
                    )
                }
            </Tabs>
        </div>
    )
}