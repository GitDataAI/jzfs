import {useNavigate} from "react-router-dom";
import useUser from "@/state/useUser.tsx";
import {
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownSection,
    DropdownTrigger, Input,
    Link,
    User
} from "@heroui/react";
import {IoAdd} from "react-icons/io5";
import { UserApi } from "@/api/UserApi";
import {toast} from "@pheralb/toast";
import {Modal, useDisclosure} from "@heroui/modal";
import LayoutModelRepository from "@/app/Layout.Model.Repository.tsx";
import AuthLayout from "@/app/auth/layout.tsx";
import {useState} from "react";

export const Header = () => {
    const nav = useNavigate();
    const user = useUser();
    const api = new UserApi();
    const RepoModal = useDisclosure();
    const Auth = useDisclosure();
    const [AuthPosition, setAuthPosition] = useState<"login" | "apply" | "reset">("login")
    return (
        <div>
            <header className="header">
                <div className="header-title">
                    <a href="/">
                        <img src="/gitdata-ai.png" alt="logo" />
                    </a>
                </div>
                {
                    !user.isLogin ? (
                           <>
                               <div className="header-menu">
                                   <span >产品</span>
                                   <span >解决方案</span>
                                   <span >资源</span>
                                   <span >企业</span>
                               </div>
                               <div className="header-login">
                                   <span onClick={()=>{
                                       Auth.onOpen()
                                       setAuthPosition("login")
                                   }}>登录</span>
                                   <span onClick={()=>{
                                       Auth.onOpen()
                                       setAuthPosition("apply")
                                   }}>注册</span>
                               </div>
                               <div style={{
                                   position: "fixed",
                                   zIndex: 9999
                               }}>
                                   <Modal
                                       backdrop="blur"
                                       isOpen={Auth.isOpen}
                                       size={"2xl"}
                                       onClose={Auth.onClose}
                                       onOpenChange={Auth.onOpenChange} >
                                       <AuthLayout position={AuthPosition} onClose={Auth.onClose} setPosition={setAuthPosition}/>
                                   </Modal>
                               </div>

                           </>
                    ):(
                        <>
                            <div className="header-menu">
                                <span onClick={()=>{nav("/explore")}}>explore</span>
                                <span  onClick={()=>{nav("/market")}}>marketplace</span>
                                <span  onClick={()=>{nav("/community")}}>community</span>
                            </div>
                            <div className={"header-right"}>
                                <Input
                                    className={"header-search"}
                                    variant="bordered"
                                    label={"Search  ⌘ K"}
                                    color={"default"}/>
                                <Dropdown>
                                    <DropdownTrigger >
                                        <div className={"header-create"}>
                                            <IoAdd />
                                        </div>
                                    </DropdownTrigger>
                                    <DropdownMenu aria-label="Static Actions">
                                        <DropdownSection showDivider>
                                            <DropdownItem onPress={()=>{
                                                RepoModal.onOpen()
                                            }} key="new_repo">New Repository</DropdownItem>
                                            <DropdownItem key="import_repo">Import Repository</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection showDivider>
                                            <DropdownItem key="new_org">New Organization</DropdownItem>
                                            <DropdownItem key="new_team">New Team</DropdownItem>
                                            <DropdownItem key="new_project">New Project</DropdownItem>
                                        </DropdownSection>
                                    </DropdownMenu>
                                </Dropdown>
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


                                <Dropdown>
                                    <DropdownTrigger>
                                        <User className={"header-card"}
                                              avatarProps={{
                                                  src: user.dash?.user.avatar
                                              }}
                                              description={
                                                  <Link style={{
                                                      cursor: "pointer"
                                                  }} isExternal onPress={()=>{
                                                      nav(user.dash!.user.username)
                                                  }} color={"primary"} size="sm">
                                                      @{user.dash?.user.username}
                                                  </Link>
                                              }
                                              name={user.dash?.user.username}
                                        />
                                    </DropdownTrigger>
                                    <DropdownMenu aria-label="Static Actions">
                                        <DropdownSection>
                                            <DropdownItem onPress={()=>{
                                                nav(user.dash!.user.username)
                                            }} key="profile">Profile</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection showDivider>
                                            <DropdownItem key="team">Team</DropdownItem>
                                            <DropdownItem key="group">Group</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection showDivider >
                                            <DropdownItem  onPress={()=>{
                                                nav(user.dash?.user.username+"?tab=reposiotry")
                                            }} key="repo">Repository</DropdownItem>
                                            <DropdownItem key="project">Project</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection showDivider >
                                            <DropdownItem onPress={()=>{
                                                nav(user.dash?.user.username+"?tab=star")
                                            }}  key="star">Star</DropdownItem>
                                            <DropdownItem onPress={()=>{
                                                nav(user.dash?.user.username+"?tab=follow")
                                            }}  key="follow">Follow</DropdownItem>

                                        </DropdownSection>
                                        <DropdownSection showDivider >
                                            <DropdownItem key="setting" onPress={()=>{
                                                nav(user.dash?.user.username+"?tab=setting")
                                            }} >Setting</DropdownItem>
                                            <DropdownItem key="help">Help & Feedback</DropdownItem>
                                            <DropdownItem key="sync" onPress={()=>{
                                                if (user.dash?.user.username) {
                                                    api.DashBoredData(user.dash?.user.username)
                                                        .then((res)=>{
                                                            const json = JSON.parse(res.data);
                                                            if (json.code === 200 && json.data && res.status) {
                                                                user.setDashBored(json.data)
                                                                toast.success({
                                                                    text: "Sync Data",
                                                                    description: "Sync Success",
                                                                });
                                                                window.location.reload();
                                                            }else {
                                                                toast.error({
                                                                    text: "Sync Data",
                                                                    description: "Sync Failed",
                                                                });
                                                            }
                                                        })
                                                }
                                            }}>Sync Data</DropdownItem>
                                            <DropdownItem key="logout" onPress={()=>{
                                                user.logout();
                                                nav("/")
                                            }} className="text-danger" color="danger">Logout</DropdownItem>
                                        </DropdownSection>
                                    </DropdownMenu>
                                </Dropdown>
                            </div>
                        </>
                    )
                }
            </header>
        </div>
    )
}