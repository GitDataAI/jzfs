import {Tab, Tabs} from "@heroui/tabs";
import {useSearchParams} from "react-router-dom";
import {useEffect, useState} from "react";

export const UserHeader = (props: { setTab: (arg0: string)=> void }) => {
    const [Query , setQuery] = useSearchParams();
    const [ Tabes, setTab ] = useState("active");
    useEffect(()=>{
       if (!Query.get("tab")){
           Query.set("tab","active")
           setQuery(Query)
           setTab("active")
           props.setTab("active")
       } else {
           setTab(Query.get("tab") as string)
           props.setTab(Query.get("tab") as string)
       }
    },[Query])
    return (
        <div className="user-header">
            <Tabs variant="bordered" className="user-header-tabs" onSelectionChange={(x)=>{
                setTab(x.toString());
                Query.set("tab",x.toString())
                setQuery(Query)
                props.setTab(x.toString())
            }} selectedKey={Tabes}>
                <Tab key="active" title="Active"/>
                <Tab key="reposiotry" title="Reposiotry"/>
                <Tab key="package" title="Package"/>
                <Tab key="release" title="Release"/>
                <Tab key="star" title="Star"/>
                <Tab key="follow" title="Follow"/>
            </Tabs>
        </div>
    )
}