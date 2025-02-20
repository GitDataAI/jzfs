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
    useEffect(()=>{
        if (State.current) return;
        if (!Query.get("tab")){
            Query.set("tab","file")
            setQuery(Query)
            setTab("file")
            props.setTab("file")
            State.current = true
        } else {
            setTab(Query.get("tab") as string)
            props.setTab(Query.get("tab") as string)
            State.current = true
        }
    },[props])
    useEffect(()=>{
        setTab(Query.get("tab") as string)
        props.setTab(Query.get("tab") as string)
    },[Query, State, props, setQuery])
    return (
        <div className="repo-header">
            <div className="repo-header-tab">
                <Tabs variant="bordered" className="user-header-tabs" onSelectionChange={(x)=>{
                    setTab(x.toString());
                    Query.set("tab",x.toString())
                    setQuery(Query)
                    props.setTab(x.toString())
                }} selectedKey={Tabes}>
                    <Tab key="file" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-cyan-100 text-success">
                                <FileIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Files</span>
                        </div>
                    }/>
                    <Tab key="wiki" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-white text-foreground">
                                <WikiIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Wiki</span>
                        </div>
                    }/>
                    <Tab key="issues" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_issue} size="sm">

                            <IconWrapper className="bg-success/10 text-success">
                                <BugIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Issues</span>
                        </Badge>
                    }/>

                    <Tab key="pr" title={
                        <Badge color="primary" placement={"top-right"} content={props.info.nums_pullrequest} size="sm">

                            <IconWrapper className="bg-primary/10 text-primary">
                                <PullRequestIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Pull Request</span>
                        </Badge>
                    }/>
                    <Tab key="discission" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-secondary/10 text-secondary">
                                <ChatIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Discission</span>
                        </div>
                    }/>
                    <Tab key="action" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-warning/10 text-warning">
                                <PlayCircleIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Actions</span>
                        </div>
                    }/>
                    <Tab key="project" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-default/50 text-foreground">
                                <LayoutIcon className="text-lg " />
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Project</span>
                        </div>
                    }/>
                    <Tab key="setting" title={
                        <div style={{
                            display: "flex"
                        }}>
                            <IconWrapper className="bg-amber-400 text-foreground">
                                <SettingIcon className="text-lg text-amber-800"/>
                            </IconWrapper>
                            <span style={{
                                padding: "3px"
                            }}>Setting</span>
                        </div>
                    }/>
                </Tabs>

            </div>

        </div>
    )
}