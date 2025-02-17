import {Tab, Tabs} from "@heroui/tabs";
import {useSearchParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {Repository} from "@/types.ts";
import {Badge} from "@heroui/react";

export const RepoHeader = (props: { setTab: (arg0: string) => void, info: Repository, owner: string, repo: string }) => {
    const [Query , setQuery] = useSearchParams();
    const [ Tabes, setTab ] = useState("file");
    useEffect(()=>{
        if (!Query.get("tab")){
            Query.set("tab","file")
            setQuery(Query)
            setTab("file")
            props.setTab("file")
        } else {
            setTab(Query.get("tab") as string)
            props.setTab(Query.get("tab") as string)
        }
    },[Query, props, setQuery])
    return (
        <div className="repo-header">
            <div className="repo-header-tab">
                <Tabs variant="bordered" className="user-header-tabs" onSelectionChange={(x)=>{
                    setTab(x.toString());
                    Query.set("tab",x.toString())
                    setQuery(Query)
                    props.setTab(x.toString())
                }} selectedKey={Tabes}>
                    <Tab key="file" title="File"/>
                    <Tab key="wiki" title="wiki"/>
                    <Tab key="issues" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_issue} size="sm">
                        <span style={{
                            padding: "3px"
                        }}>issues</span>
                        </Badge>
                    }/>


                    <Tab key="pr" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_pullrequest} size="sm">
                        <span style={{
                            padding: "3px"
                        }}>pull request</span>
                        </Badge>
                    }/>
                    <Tab key="discission" title="discission"/>
                    <Tab key="action" title="action"/>
                    <Tab key="project" title="project"/>
                    <Tab key="insights" title="insights"/>
                    <Tab key="setting" title="setting"/>
                </Tabs>

            </div>

        </div>
    )
}