import {Tab, Tabs} from "@heroui/tabs";
import {useSearchParams} from "react-router-dom";
import {useEffect, useRef, useState} from "react";
import {Repository} from "@/types.ts";
import {Badge} from "@heroui/react";
import {
    BugIcon,
    ChatIcon, FileIcon,
    IconWrapper,
    LayoutIcon,
    PlayCircleIcon,
    PullRequestIcon,
    SettingIcon, WikiIcon
} from "@/app/repo/Repo.Icons.tsx";

export const RepoHeader = (props: { setTab: (arg0: string) => void, info: Repository, owner: string, repo: string }) => {
    const [Query , setQuery] = useSearchParams();
    const [ Tabes, setTab ] = useState("file");
    const State = useRef(false);
    // useEffect(()=>{
    //     if (State.current) return;
    //     if (!Query.get("tab")){
    //         Query.set("tab","file")
    //         setQuery(Query)
    //         setTab("file")
    //         props.setTab("file")
    //         State.current = true
    //     } else {
    //         setTab(Query.get("tab") as string)
    //         props.setTab(Query.get("tab") as string)
    //         State.current = true
    //     }
    // },[props])
    useEffect(()=>{
        setTab(Query.get("tab") as string)
        props.setTab(Query.get("tab") as string)
    },[Query, State, props, setQuery])
    return (
        <div className="repo-header">
            <div className="repo-header-tab">
                <Tabs variant="bordered" className="repo-header-tabs" onSelectionChange={(x)=>{
                    setTab(x.toString());
                    Query.set("tab",x.toString())
                    setQuery(Query)
                    props.setTab(x.toString())
                }} selectedKey={Tabes}>
                    <Tab key="file" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="">
                                <FileIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Files</span>
                            </IconWrapper>

                        </div>
                    }/>
                    <Tab key="wiki" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-white text-foreground">
                                <WikiIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Wiki</span>
                            </IconWrapper>

                        </div>
                    }/>
                    <Tab key="issues" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_issue} size="sm">

                            <IconWrapper className="">
                                <BugIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Issues</span>
                            </IconWrapper>
                        </Badge>
                    }/>

                    <Tab key="pr" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_pullrequest} size="sm">

                            <IconWrapper className="">
                                <PullRequestIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Pull Request</span>
                            </IconWrapper>
                        </Badge>
                    }/>
                    <Tab key="discission" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="">
                                <ChatIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Discussion</span>
                            </IconWrapper>

                        </div>
                    }/>
                    <Tab key="action" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="">
                                <PlayCircleIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Actions</span>
                            </IconWrapper>
                        </div>
                    }/>
                    <Tab key="project" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="">
                                <LayoutIcon className="text-lg " />
                                <span style={{
                                    padding: "3px"
                                }}>Project</span>
                            </IconWrapper>
                        </div>
                    }/>
                    <Tab key="setting" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="">
                                <SettingIcon className="text-lg "/>
                                <span style={{
                                    padding: "3px"
                                }}>Setting</span>
                            </IconWrapper>
                        </div>
                    }/>
                </Tabs>

            </div>

        </div>
    )
}