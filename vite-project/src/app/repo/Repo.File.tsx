import {useEffect, useRef, useState} from "react";
import {Blob, Branches, Commits, DBCommit, Repository, Tree} from "@/types.ts";
import {
    Avatar,
    Badge,
    BreadcrumbItem,
    Breadcrumbs,
    Button,
    Card,
    CardBody,
    CardHeader,
    Code,
    Select,
    SelectItem,
} from "@heroui/react";
import {RepoApi} from "@/api/RepoApi.tsx";
import {Modal, ModalContent, ModalHeader, useDisclosure} from "@heroui/modal";
import {RepoClone} from "@/app/repo/Repo.Clone.tsx";
import {RepoEmpty} from "@/app/repo/Repo.Empty.tsx";
import {RepoREADME} from "@/app/repo/Repo.README.tsx";
import {Tab, Tabs} from "@heroui/tabs";
import {useNavigate} from "react-router-dom";
import dayjs from "dayjs"
import relativeTime from "dayjs/plugin/relativeTime"
import useUser from "@/state/useUser.tsx";
import {toast} from "@pheralb/toast";
import {RepoFork} from "@/app/repo/Repo.Fork.tsx";


dayjs.extend(relativeTime);

interface RepoFileProps {
    info: Repository,
    owner: string,
    repo: string,
    upDate: () => void
}

interface BhtcItem {
    index: string,
    branch: Branches,
    commit: Commits,
    tree: Tree
}

const RepoFile = (props: RepoFileProps) => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [_Bhtc, setBhtc] = useState<BhtcItem[]>([])
    const [Tree, setTree] = useState<Tree | null>(null)
    const [DefaultBranch, setDafaultBranch] = useState<Branches | null>()
    const [Load, setLoad] = useState(false)
    const [Blob, setBlob] = useState<Blob | null>(null)
    const [Branches, setBranches] = useState<Branches[]>([])
    const [Head, setHead] = useState<DBCommit | null>(null)
    const user = useUser();

    const [Edit] = useState({
        edit: false
    })
    const api = new RepoApi();
    const [HttpURL, setHttpURL] = useState("");
    const [SSHURL, setSSHURL] = useState("");
    const Clone = useDisclosure();
    const Exec = useRef(false);
    const [README, setREADME] = useState<Uint8Array | null>(null);
    const nav = useNavigate();
    const fork = useDisclosure();
    useEffect(() => {
        const abortController = new AbortController();
        const {signal} = abortController;

        const fetchData = async () => {
            try {
                setLoad(false);
                const httpURL = `https://${window.location.host}/git/${encodeURIComponent(props.owner)}/${encodeURIComponent(props.repo)}.git`;
                setHttpURL(httpURL);
                const sshURL = `ssh://git@${window.location.host}:2322/${encodeURIComponent(props.owner)}/${encodeURIComponent(props.repo)}.git`;
                setSSHURL(sshURL);

                const res = await api.Bhtc(props.owner, props.repo);
                if (res.status !== 200 || !res.data) return;

                const jsonData: Blob = JSON.parse(res.data).data;
                setBlob(jsonData);

                const branches: Branches[] = Object.keys(jsonData).map(key => JSON.parse(key));
                setBranches(branches)
                const branchPromises = branches.map(async (branch) => {
                    const treeRes = await api.Tree(props.owner, props.repo, branch.name, branch.head);
                    if (treeRes.status !== 200 || !treeRes.data) return null;

                    const jsonb: Tree | undefined = JSON.parse(treeRes.data).data;
                    if (!jsonb) return null;
                    const defbr = branches.find(value => value.name === props.info.default_branch);

                    if (!DefaultBranch) {
                        if (defbr && branch.name === props.info.default_branch){
                            setDafaultBranch(defbr);
                            setTree(jsonb);
                        } else if (!defbr){
                            setDafaultBranch(branch);
                            setTree(jsonb);
                        }
                        const readmeChild = jsonb.child.find(child => child.name === "README.md");
                        if (readmeChild) {
                            const readmeRes = await api.File(props.owner, props.repo, "README.md", branch.head);
                            if (readmeRes.status === 200 && readmeRes.data) setREADME(readmeRes.data);
                        }
                    }

                    return {
                        index: branch.name,
                        branch,
                        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                        // @ts-expect-error
                        commit: jsonData[JSON.stringify(branch)],
                        tree: jsonb
                    };
                });

                const validBranches = (await Promise.all(branchPromises)).filter(Boolean) as BhtcItem[];
                setBhtc(prev => [...prev, ...validBranches]);

            } catch (err) {
                if (!signal.aborted) console.error('Fetch error:', err);
            } finally {
                if (!signal.aborted) setLoad(true);
            }
        };

        if (!Exec.current) {
            fetchData();
            Exec.current = true;
        }

        return () => abortController.abort();
    }, [props.owner, props.repo]);

    useEffect(() => {

    }, [Blob]);
    useEffect(() => {
        const handle = async () => {
            if (DefaultBranch) {
                const head = DefaultBranch.head;
                const commitRes = await api.OneCommit(props.owner, props.repo, props.info.default_branch, head);
                if (commitRes.status !== 200 && !commitRes.data) return;
                if (commitRes.data) {
                    setHead(JSON.parse(commitRes.data).data);
                }
            }
        }
        handle().catch().then()
    }, [DefaultBranch]);
    const UpdateTree = async (default_branches: Branches) => {
        if (!Load) return;
        const treeRes = await api.Tree(props.owner, props.repo, default_branches.name, default_branches.head);
        if (treeRes.status !== 200 || !treeRes.data) return;
        const jsonb: Tree | undefined = JSON.parse(treeRes.data).data;
        if (!jsonb) return;
        setTree(jsonb);
        const readmeChild = jsonb.child.find(child => child.name === "README.md");
        if (readmeChild) {
            const readmeRes = await api.File(props.owner, props.repo, "README.md", default_branches.head);
            if (readmeRes.status === 200 && readmeRes.data) setREADME(readmeRes.data);
        }
    }
    return (
        <div className="repo-file repo-bodt">
            <div style={{
                zIndex: 999
            }}>
                <Modal
                    backdrop="blur"
                    isOpen={fork.isOpen}
                    size={"2xl"}
                    onOpenChange={fork.onOpenChange}
                    onClose={fork.onClose}
                >
                    <ModalContent>
                        <ModalHeader>
                            Fork Repository for &nbsp;<Code size="lg">{props.owner}/{props.repo}</Code>
                        </ModalHeader>
                        <RepoFork owner={props.owner} repo={props.repo} close={fork.onClose}/>
                    </ModalContent>
                </Modal>
            </div>
            {
                Edit.edit ? (
                    <>

                    </>
                ) : (
                    <> {
                        (Load) && (
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
                                        <RepoClone owner={props.owner} repo={props.repo} http={HttpURL} ssh={SSHURL}/>
                                    </Modal>
                                </div>
                                <div className="repo-header-title">
                                    <Breadcrumbs key={"bordered"} variant={"light"} className="bordered" separator="/">
                                        <BreadcrumbItem separator={""}>
                                            <Avatar color={"default"} radius={"sm"}
                                                    src={props.info.avatar || "https://cdn.iconscout.com/icon/premium/png-128-thumb/repository-4-559990.png"}/>
                                        </BreadcrumbItem>
                                        <BreadcrumbItem onClick={() => {
                                            nav("/" + props.owner)
                                        }}>{props.owner}</BreadcrumbItem>
                                        <BreadcrumbItem>{props.repo}</BreadcrumbItem>
                                    </Breadcrumbs>
                                    <div className="repo-header-action">
                                        <Badge color={"primary"} content={props.info.nums_fork}>
                                            <Button color="primary" variant="faded" onPress={fork.onOpen}>
                                                fork
                                            </Button>
                                        </Badge>
                                        <Badge color={"primary"} content={props.info.nums_star}>
                                            <Button color="primary" variant="faded">
                                                {
                                                    user.dash ? (
                                                        <div onClick={() => {
                                                            api.Star(props.owner, props.repo).then(res => {
                                                                const json = JSON.parse(res.data);
                                                                if (res.status === 200 && json.code === 200) {
                                                                    user.syncData()
                                                                    toast.success({
                                                                        text: "Operation successful"
                                                                    })
                                                                    props.upDate();
                                                                }
                                                            })
                                                        }}>
                                                            {
                                                                user.dash?.stars.find((value) => value.repository_id === props.info.uid) ? (
                                                                    <>
                                                                        unstar
                                                                    </>
                                                                ) : (
                                                                    <>
                                                                        star
                                                                    </>
                                                                )
                                                            }
                                                        </div>
                                                    ) : (
                                                        <>star</>
                                                    )
                                                }
                                            </Button>
                                        </Badge>
                                        <Badge color={"primary"} content={props.info.nums_watch}>
                                            <Button color="primary" variant="faded">
                                                {
                                                    user.dash ? (
                                                        <div onClick={() => {
                                                            api.Watch(props.owner, props.repo, 1).then(res => {
                                                                const json = JSON.parse(res.data);
                                                                if (res.status === 200 && json.code === 200) {
                                                                    user.syncData()
                                                                    toast.success({
                                                                        text: "Operation successful"
                                                                    })
                                                                    props.upDate();
                                                                }
                                                            })
                                                        }}>
                                                            {
                                                                user.dash?.watch.find((value) => value.repository_id === props.info.uid) ? (
                                                                    <>
                                                                        unwatch
                                                                    </>
                                                                ) : (
                                                                    <>
                                                                        watch
                                                                    </>
                                                                )
                                                            }
                                                        </div>
                                                    ) : (
                                                        <>watch</>
                                                    )
                                                }
                                            </Button>
                                        </Badge>
                                    </div>
                                </div>
                                <div className="repo-file-body">
                                    <div className="repo-file-body-main">
                                        <Card>
                                            <CardHeader className="repo-file-body-main-header">
                                                <div className="repo-file-body-main-header-left">
                                                    {
                                                        DefaultBranch && (
                                                            <Select
                                                                showScrollIndicators={false}
                                                                isRequired
                                                                disallowEmptySelection
                                                                defaultSelectedKeys={[DefaultBranch!.name]}
                                                                selectedKeys={[DefaultBranch!.name]} value={DefaultBranch!.name}
                                                                className="branch-select"
                                                                onSelectionChange={(key) => {
                                                                    const currentKey = key.currentKey;
                                                                    if (currentKey) {
                                                                        const default_branches = Branches.find((value) => value.name === currentKey);
                                                                        if (default_branches) {
                                                                            setDafaultBranch(default_branches);
                                                                            UpdateTree(default_branches).then(() => {
                                                                            }).catch(() => {
                                                                            })
                                                                        }
                                                                    }
                                                                }}>
                                                                {
                                                                    Branches.map(value => {
                                                                        return (
                                                                            <SelectItem key={value.name}>
                                                                                {value.name}
                                                                            </SelectItem>
                                                                        )
                                                                    })
                                                                }
                                                            </Select>
                                                        )
                                                    }
                                                    {
                                                        (Head !== null) && (
                                                            <div className="head-message">
                                                                {Head.message}
                                                                {/*// TODO Status*/}
                                                            </div>
                                                        )
                                                    }
                                                </div>
                                                <div className="repo-file-body-main-header-right">
                                                    <Button
                                                        color="success"
                                                        style={{
                                                            color: "white"
                                                        }}
                                                        onPress={() => {
                                                            Clone.onOpen()
                                                        }}
                                                    >
                                                        Clone
                                                    </Button>
                                                </div>
                                            </CardHeader>
                                            <CardBody className="repo-file-body-file">
                                                <Card className="repo-file-body-file-left">
                                                    {
                                                        Tree ? (
                                                            <CardBody>
                                                                <RepoFileList tree={Tree}/>
                                                            </CardBody>
                                                        ) : (
                                                            <>
                                                                <CardBody>
                                                                    <RepoEmpty/>
                                                                </CardBody>
                                                            </>
                                                        )
                                                    }

                                                </Card>
                                                <Card className="repo-file-body-file-right">
                                                    <CardBody>

                                                    </CardBody>
                                                </Card>
                                            </CardBody>
                                        </Card>
                                    </div>
                                    {
                                        (README) && (
                                            <Card style={{
                                                marginTop: "1rem"
                                            }}>
                                                <CardBody>
                                                    <Tabs>
                                                        {
                                                            README && (
                                                                <Tab key="readme" title="README">
                                                                    <RepoREADME file={README}/>
                                                                </Tab>
                                                            )
                                                        }
                                                    </Tabs>
                                                </CardBody>
                                            </Card>
                                        )
                                    }
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

function FileItem({tree}: { tree: Tree }) {
    const commit: Commits | undefined = tree.commit.sort((a, b) => {
        if (a.time > b.time) {
            return -1;
        } else if (a.time < b.time) {
            return 1;
        } else {
            return 0;
        }
    })[0];
    if (commit) {
        commit.msg = commit.msg.substring(0, 50)
    }
    const relative_time = () => {
        if (commit) {
            const date = new Date(Number(commit.time) * 1000);
            const to_now = dayjs().to(dayjs(date));
            return <>{to_now}</>
        } else {
            return <>N/A</>
        }
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
                        <div>
                            {relative_time()}
                        </div>
                    </div>
                )
            }
        </div>
    );
}

function Folder({tree}: { tree: Tree }) {
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
                <div className="folder-children" style={{marginLeft: '20px'}}>
                    {tree.child.map((item, index) => (
                        <div key={item.id || index}>
                            {item.is_dir ? (
                                <Folder tree={item}/>
                            ) : (
                                <FileItem tree={item}/>
                            )}
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
}

function RepoFileList({tree}: { tree: Tree }) {
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
                        <Folder tree={item}/>
                    ) : (
                        <FileItem tree={item}/>
                    )}
                </div>
            ))}
        </div>
    );
}

export default RepoFile