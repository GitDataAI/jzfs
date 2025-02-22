import {Repository, UserDashBored} from "@/types.ts";
import {
    BreadcrumbItem,
    Breadcrumbs,
    Button,
    Card,
    CardBody,
    CardFooter,
    Input,
    Select,
    SelectItem,
    User
} from "@heroui/react";
import {IoIosGitBranch, IoIosGitCommit, IoIosGitPullRequest} from "react-icons/io";
import {FaTag} from "react-icons/fa";
import {AiOutlineNodeIndex} from "react-icons/ai";
import {GoIssueOpened, GoRepoForked} from "react-icons/go";
import { PiSwatches } from "react-icons/pi";
import {CiStar} from "react-icons/ci";
import {  Listbox,  ListboxItem} from "@heroui/listbox";
import {useEffect, useState} from "react";
import {Modal, useDisclosure} from "@heroui/modal";
import LayoutModelRepository from "@/app/Layout.Model.Repository.tsx";
import {useNavigate} from "react-router-dom";
import useUser from "@/state/useUser.tsx";


// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
export const ListboxWrapper = ({children}) => (
    <div className="w-full max-w-[260px] border-small px-1 py-2 rounded-small border-default-200 dark:border-default-100">
        {children}
    </div>
);
const UserRepository = (props: {props: UserDashBored}) => {
    const [Repos, setRepos] = useState<Repository[]>([]);
    const RepoModal = useDisclosure();
    const nav = useNavigate();
    const [Type, setType] = useState("all");
    const user = useUser();
    useEffect(() => {
        setRepos(props.props.repos)
    }, [props.props.repos]);
    return(
        <div className="user-repo user-bodt">
            <ListboxWrapper>
                <Listbox aria-label="Actions" onAction={(key) => {
                    setType(key.toString())
                    switch (key) {
                        case "all":
                            setRepos(props.props.repos)
                            break;
                        case "public":
                            setRepos(props.props.repos.filter((value) => {
                                return !value.visibility
                            }))
                            break;
                        case "private":
                            setRepos(props.props.repos.filter((value) => {
                                return value.visibility
                           }))
                            break;
                        case "fork":
                            setRepos(props.props.repos.filter((value) => {
                                return value.fork !== null
                            }))
                            break;
                    }
                }}>
                    <ListboxItem key="all">all</ListboxItem>
                    <ListboxItem key="public">public</ListboxItem>
                    <ListboxItem key="private">private</ListboxItem>
                    <ListboxItem key="source">source</ListboxItem>
                    <ListboxItem key="fork">fork</ListboxItem>
                </Listbox>
            </ListboxWrapper>
            <div className="user-repo-main">
                <h1>
                    {Type.toUpperCase()}
                </h1>
                <div className="user-repo-main-header">
                    <Input label={"Search"} className="user-repo-search"/>
                    <Select defaultSelectedKeys={['rand']} label="Sort" className="user-repo-sort" size={"sm"} onSelectionChange={(x)=>{
                        const sort = x.currentKey?.toString();
                        if (sort) {
                            console.log(sort)
                            switch (sort) {
                                case "rand":
                                    setRepos(props.props.repos.sort(()=>{
                                        return Math.random() - 0.5
                                    }))
                                    break;
                                case "name":
                                    setRepos(props.props.repos.sort((a, b)=>{
                                        const idx_a = a.name[0];
                                        const idx_b = b.name[0];
                                        if (idx_a > idx_b) {
                                            return 1
                                        } else if (idx_a < idx_b) {
                                            return -1
                                        } else {
                                            return 0
                                        }
                                    }))
                                    break;
                                case "star":
                                    setRepos(props.props.repos.sort((a, b)=>{
                                        const idx_a = a.nums_star;
                                        const idx_b = b.nums_star;
                                        if (idx_a > idx_b) {
                                            return 1
                                        } else if (idx_a < idx_b) {
                                            return -1
                                        } else {
                                            return 0
                                        }
                                    }))
                                    break;
                                case "watch":
                                    setRepos(props.props.repos.sort((a, b)=>{
                                        return b.nums_watch - a.nums_watch
                                    }))
                                    break;
                                case "fork":
                                    setRepos(props.props.repos.sort((a, b)=>{
                                        return b.nums_fork - a.nums_fork
                                    }))
                                    break;
                            }
                        }
                    }}>
                        <SelectItem key="rand" >rand</SelectItem>
                        <SelectItem key="name" >name</SelectItem>
                        <SelectItem key="star" >star</SelectItem>
                        <SelectItem key="watch" >watch</SelectItem>
                        <SelectItem key="fork" >fork</SelectItem>
                    </Select>
                    <Select size={"sm"} label="up-to-date" className="user-repo-sort">
                        <SelectItem key="new">recent</SelectItem>
                        <SelectItem key="copy">old</SelectItem>
                    </Select>
                    {
                        (user.user && (user.user.uid === props.props.user.uid)) && (
                            <Button onPress={RepoModal.onOpen} className="user-repo-sort btn" color="success">
                                New Repository
                            </Button>
                        )
                    }
                </div>
                <div className="user-repo-list">
                {
                    Repos.map((value, index)=>{
                        return(
                            <Card key={value.uid} className="user-repo-item">
                                <CardBody onClick={()=>{
                                    nav("/"+props.props.user.username + "/" + value.name)
                                }}>
                                    <User
                                        avatarProps={{
                                            src:  value.avatar || "https://cdn.iconscout.com/icon/premium/png-128-thumb/repository-4-559990.png",
                                            radius:"sm"
                                        }}
                                        description={value.description}
                                        name={
                                            <Breadcrumbs variant={"bordered"} key={index}>
                                                <BreadcrumbItem>{props.props.user.username}</BreadcrumbItem>
                                                <BreadcrumbItem>{value.name}</BreadcrumbItem>
                                            </Breadcrumbs>
                                        }
                                    />
                                    <div className="user-repo-main-infos">
                                        update: {value.updated_at.toString()}
                                    </div>
                                </CardBody>
                                <CardFooter>
                                    <div className="footer">
                                        <span className="text-sm"><IoIosGitBranch /> {value.nums_branch}</span>
                                        <span className="text-sm"><FaTag /> {value.nums_tag}</span>
                                        <span className="text-sm"><AiOutlineNodeIndex /> {value.nums_release}</span>
                                        <span className="text-sm"><IoIosGitCommit /> {value.nums_commit}</span>
                                        <span className="text-sm"><IoIosGitPullRequest /> {value.nums_pullrequest}</span>
                                        <span className="text-sm"><GoIssueOpened /> {value.nums_issue}</span>
                                        <span className="text-sm"><PiSwatches /> {value.nums_watch}</span>
                                        <span className="text-sm"><CiStar /> {value.nums_star}</span>
                                        <span className="text-sm"><GoRepoForked /> {value.nums_fork}</span>
                                    </div>
                                </CardFooter>
                            </Card>
                        )
                    })
                }
                </div>
                <div style={{
                    position: "fixed",
                    zIndex: 9999
                }}>
                    <Modal
                        backdrop="blur"
                        isOpen={RepoModal.isOpen}
                        size={"2xl"}
                        onOpenChange={RepoModal.onOpenChange} >
                        <LayoutModelRepository onClose={RepoModal.onClose}/>
                    </Modal>
                </div>
            </div>
        </div>
    )
}

export default UserRepository