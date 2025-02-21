import {Tab, Tabs} from "@heroui/tabs";
import {useSearchParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {VscLayersActive} from "react-icons/vsc";
import {IconWrapper} from "@/app/repo/Repo.Icons.tsx";
import {RiGitRepositoryLine, RiUserFollowLine} from "react-icons/ri";
import {GoPackage} from "react-icons/go";
import {MdProductionQuantityLimits} from "react-icons/md";
import {CiSettings, CiStar} from "react-icons/ci";

export const UserHeader = (props: { setTab: (arg0: string)=> void }) => {
    const [Query , setQuery] = useSearchParams();
    const [ Tabes, setTab ] = useState("active");
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
                        </IconWrapper>
                        <span className="ml-2">Active</span>
                    </div>
                }/>
                <Tab key="reposiotry" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <RiGitRepositoryLine />
                        </IconWrapper>
                        <span className="ml-2">Reposiotry</span>
                    </div>
                }/>
                <Tab key="package" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <GoPackage />
                        </IconWrapper>
                        <span className="ml-2">Package</span>
                    </div>
                }/>
                <Tab key="product" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <MdProductionQuantityLimits />
                        </IconWrapper>
                        <span className="ml-2">Product</span>
                    </div>
                }/>
                <Tab key="star" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <CiStar />
                        </IconWrapper>
                        <span className="ml-2">Star</span>
                    </div>
                }/>
                <Tab key="follow" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <RiUserFollowLine />
                        </IconWrapper>
                        <span className="ml-2">Follow</span>
                    </div>
                }/>
                <Tab key="setting" title={
                    <div className="flex items-center">
                        <IconWrapper className="  text-black">
                            <CiSettings />
                        </IconWrapper>
                        <span className="ml-2">Setting</span>
                    </div>
                }/>
            </Tabs>
        </div>
    )
}