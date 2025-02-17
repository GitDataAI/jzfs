import {useEffect, useState} from "react";
import {Blob, Branches, Commits, Repository, Tree} from "@/types.ts";
import {Avatar, Badge, BreadcrumbItem, Breadcrumbs, Button} from "@heroui/react";
import {RepoApi} from "@/api/RepoApi.tsx";

interface RepoFileProps {
    info: Repository,
    owner: string,
    repo: string
}
interface BhtcItem {
    index: string,
    branch: Branches,
    commit: Commits,
    tree: Tree
}
const RepoFile = (props: RepoFileProps) => {
    const [Bhtc, setBhtc] = useState<BhtcItem[]>([])
    const [Tree, setTree] = useState< Tree | null>(null)
    const [DefaultBranch, setDafaultBranch] = useState<Branches | null>()
    const [Load, setLoad] = useState(false)
    const [Edit] = useState({
        edit: false
    })
    const api = new RepoApi();
    useEffect(() => {
        // console.log(props)
        // setBhtc([])
        // setDafaultBranch(null)
        // setTree(null)
        // setLoad(false);

        api.Bhtc(props.owner, props.repo)
            .then(res=>{
                if (res.status === 200 && res.data){
                    const json:Blob = JSON.parse(res.data).data;
                    for (const jsonKey in json) {
                        const branch: Branches =  JSON.parse(jsonKey);
                        api.Tree(props.owner, props.repo, branch.name,branch.head)
                            .then(res=>{
                                const jsonb:Tree | undefined = JSON.parse(res.data).data;
                                if (res.status === 200 && res.data && jsonb){
                                    if (!DefaultBranch){
                                        setDafaultBranch(branch)
                                        setTree(jsonb)
                                    }
                                    if (Bhtc.find((value) => value.branch.name === branch.name)){
                                        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                                        // @ts-expect-error
                                        setBhtc((prev) => [...prev, {index: branch.name,branch: branch, commit: json[jsonKey], tree: jsonb}]);
                                        console.log("skip")
                                    }
                                    setLoad(true);
                                }
                            })
                    }
                }
            })

    }, [props]);
    return (
        <div className="repo-file repo-bodt">
            {
                Edit.edit ? (
                    <>

                    </>
                ) : (
                   <> {
                       (Load && Tree && Bhtc && DefaultBranch) && (
                        <>
                            <div className="repo-header-title">
                                <Breadcrumbs key={"bordered"} variant={"light"} className="bordered"  separator="/">
                                    <BreadcrumbItem separator={""}>
                                        <Avatar color={"default"} radius={"sm"} src={props.info.avatar || "https://cdn.iconscout.com/icon/premium/png-128-thumb/repository-4-559990.png"}/>
                                    </BreadcrumbItem>
                                    <BreadcrumbItem>{props.owner}</BreadcrumbItem>
                                    <BreadcrumbItem>{props.repo}</BreadcrumbItem>
                                </Breadcrumbs>
                                <div className="repo-header-action">
                                    <Badge color={"primary"} content={props.info.nums_fork}>
                                        <Button  color="primary" variant="faded">
                                            fork
                                        </Button>
                                    </Badge>
                                    <Badge color={"primary"} content={props.info.nums_star}>
                                        <Button  color="primary" variant="faded">
                                            star
                                        </Button>
                                    </Badge>
                                    <Badge color={"primary"} content={props.info.nums_watch}>
                                        <Button  color="primary" variant="faded">
                                            watch
                                        </Button>
                                    </Badge>
                                </div>
                            </div>
                            <div className="repo-file-body">
                                <div className="repo-file-body-header">
                                    {/*<Select defaultSelectedKeys={DefaultBranch.name} selectedKeys={DefaultBranch.name} value={DefaultBranch.name} label={"Branch"}>*/}
                                    {/*    {*/}
                                    {/*        Bhtc.map((value, index)=>{*/}
                                    {/*            return (*/}
                                    {/*                <SelectItem key={index+"br"} value={value.branch.name}>*/}
                                    {/*                    {value.branch.name}*/}
                                    {/*                </SelectItem>*/}
                                    {/*            )*/}
                                    {/*        })*/}
                                    {/*    }*/}
                                    {/*</Select>*/}
                                </div>
                                <div className="repo-file-body-main">
                                    <RepoFileList tree={Tree}/>
                                </div>
                            </div>
                        </>
                        )
                    }
                   </>
                )
            }
        </div>
    )
}

function FileItem({ tree }: { tree: Tree }) {
    const commit:Commits | undefined = tree.commit.sort((a, b) => {
        if (a.time > b.time) {
            return -1;
        } else if (a.time < b.time) {
            return 1;
        } else {
            return 0;
        }
    })[0];
    if (commit){
        // 前10个字符
        commit.msg = commit.msg.substring(0, 50);
    }
    return (
        <div className="file-item">
            <div>
                <span>📄 </span>
                <a href={tree.id}>{tree.name}</a>
            </div>
            {
                commit && (
                    <div className={"file-item-commit"}>
                        <div>{commit.msg}</div>
                        <div>{commit.time}</div>
                    </div>
                )
            }
        </div>
    );
}

function Folder({ tree }: { tree: Tree }) {
    const [isExpanded, setIsExpanded] = useState(false);

    const toggleFolder = () => {
        setIsExpanded(!isExpanded);
    };
    tree.child.sort((a, b) => {
        if (a.is_dir && !b.is_dir) {
            return -1;
        } else if (!a.is_dir && b.is_dir) {
            return 1;
        } else {
            return a.name.localeCompare(b.name);
        }
    })
    return (
        <div className="folder-container">
            <div
                className="folder-header"
                onClick={toggleFolder}
                role="button"
                tabIndex={0}
                aria-expanded={isExpanded}
            >
        <span className="folder-icon">
          {isExpanded ? '📂 ' : '📁 '}
            </span>
                <span className="folder-name">{tree.name}</span>
            </div>

            {isExpanded && tree.child && (
                <div className="folder-children" style={{ marginLeft: '20px' }}>
                    {tree.child.map((item, index) => (
                        <div key={item.id || index}>
                            {item.is_dir ? (
                                <Folder tree={item} />
                            ) : (
                                <FileItem tree={item} />
                            )}
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
}
function RepoFileList({ tree }: { tree: Tree }) {
    tree.child.sort((a, b) => {
        if (a.is_dir && !b.is_dir) {
            return -1;
        } else if (!a.is_dir && b.is_dir) {
            return 1;
        } else {
            return a.name.localeCompare(b.name);
        }
    })
    return (
        <div className="file-tree">
            {tree.child?.map((item, index) => (
                <div key={item.id || index}>
                    {item.is_dir ? (
                        <Folder tree={item} />
                    ) : (
                        <FileItem tree={item} />
                    )}
                </div>
            ))}
        </div>
    );
}

export default RepoFile