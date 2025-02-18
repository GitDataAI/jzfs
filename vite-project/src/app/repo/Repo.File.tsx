import {useEffect, useRef, useState} from "react";
import {Blob, Branches, Commits, Repository, Tree} from "@/types.ts";
import {
    Avatar,
    Badge,
    BreadcrumbItem,
    Breadcrumbs,
    Button,
    Card,
    CardBody,
    CardHeader,
} from "@heroui/react";
import {RepoApi} from "@/api/RepoApi.tsx";
import {Modal, useDisclosure} from "@heroui/modal";
import {RepoClone} from "@/app/repo/Repo.Clone.tsx";

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
    const [Blob, setBlob] = useState<Blob | null>(null)
    // const [Branches, setBranches] = useState<string[]>([])
    const [Edit] = useState({
        edit: false
    })
    const api = new RepoApi();
    const [ HttpURL, setHttpURL] = useState("");
    const Clone = useDisclosure();
    const Exec = useRef(false);
    useEffect(() => {
        // console.log(props)
        // setBhtc([])
        // setDafaultBranch(null)
        // setTree(null)
        // setLoad(false);
        if (Exec.current) return;
        setHttpURL("https://" + window.location.host + "/git/" + props.owner + "/" + props.repo + ".git")
        api.Bhtc(props.owner, props.repo)
            .then(res=>{
                if (res.status === 200 && res.data){
                    const json:Blob = JSON.parse(res.data).data;
                    setBlob(json);
                    for (const jsonKey in json) {
                        const branch: Branches =  JSON.parse(jsonKey);
                        // setBranches((pre)=> [...pre, branch.name])
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
        Exec.current = true;

    }, [ props]);
    return (
        <div className="repo-file repo-bodt">
            {
                Edit.edit ? (
                    <>

                    </>
                ) : (
                   <> {
                       (Load && Tree && Bhtc && DefaultBranch && Blob) && (
                        <>
                            <div
                                style={{
                                    position: "fixed",
                                    zIndex: 9999
                                }}
                            >
                                <Modal
                                    backdrop="blur"
                                    isOpen={Clone.isOpen}
                                    size={"2xl"}
                                    onClose={Clone.onClose}
                                    onOpenChange={Clone.onOpenChange}
                                >
                                    <RepoClone owner={props.owner} repo={props.repo} http={HttpURL}/>
                                </Modal>
                            </div>
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
                                <div className="repo-file-body-main">
                                    <Card>
                                        <CardHeader className="repo-file-body-main-header">
                                           <div className="repo-file-body-main-header-left">

                                           </div>
                                            <div className="repo-file-body-main-header-right">
                                                <Button
                                                    color="success"
                                                    style={{
                                                        color: "white"
                                                    }}
                                                    onPress={()=>{
                                                        Clone.onOpen()
                                                    }}
                                                >
                                                    Clone
                                                </Button>
                                           </div>
                                        </CardHeader>
                                        <CardBody className="repo-file-body-file">
                                            <Card className="repo-file-body-file-left">
                                                <CardBody>
                                                    <RepoFileList tree={Tree}/>
                                                </CardBody>
                                            </Card>
                                            <Card className="repo-file-body-file-right">
                                                <CardBody>

                                                </CardBody>
                                            </Card>
                                        </CardBody>
                                    </Card>
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
        commit.msg = commit.msg.substring(0, 50);
    }
    return (
        <div className="file-item">
            <div>
                <span>📄 </span>
                <a>{tree.name}</a>
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