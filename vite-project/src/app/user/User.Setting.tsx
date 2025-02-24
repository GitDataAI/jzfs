import {Card, CardBody} from "@heroui/react";
import {ListboxWrapper} from "@/app/user/User.Repository.tsx";
import {Listbox, ListboxItem, ListboxSection} from "@heroui/listbox";
import React, {useEffect} from "react";
import {MdManageAccounts} from "react-icons/md";
import {SiAwssecretsmanager, SiSession} from "react-icons/si";
import {TfiEmail} from "react-icons/tfi";
import {BsShieldLockFill} from "react-icons/bs";
import {IoIosNotificationsOutline} from "react-icons/io";
import {RiClapperboardAiLine} from "react-icons/ri";
import {ImProfile} from "react-icons/im";
import {UserSettingAccess} from "@/app/user/User.Setting.Access.tsx";
import {UserDashBored} from "@/types.ts";




export const UserSetting = (props:{props: UserDashBored}) => {
    const [selectedKeys, setSelectedKeys] = React.useState(['Profile']);
    const selectedValue = React.useMemo(() => Array.from(selectedKeys).join(", "), [selectedKeys]);
    useEffect(() => {
        console.log(selectedValue)
    }, [selectedValue]);
    return(
        <div className="repo-setting">
            <div className="flex flex-col">
                <ListboxWrapper>
                    <Listbox
                        disallowEmptySelection
                        aria-label="Single selection example"
                        selectedKeys={selectedKeys}
                        selectionMode="single"
                        variant="flat"
                        onSelectionChange={(x)=>{
                            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                            // @ts-expect-error
                            setSelectedKeys(x)
                        }}
                    >
                        <ListboxSection>
                            <ListboxItem key={"Profile"}>
                                <div className="flex items-center">
                                    <ImProfile /><span className="ml-2">Profile</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection>
                            <ListboxItem key={"Account"}>
                                <div className="flex items-center">
                                    <MdManageAccounts /><span className="ml-2">Account</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection>
                            <ListboxItem key={"Appearance"}>
                                <div className="flex items-center">
                                    <RiClapperboardAiLine /><span className="ml-2">Appearance</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection>
                            <ListboxItem key={"Notifications"}>
                                <div className="flex items-center">
                                    <IoIosNotificationsOutline /><span className="ml-2">Notifications</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection title="access">
                            <ListboxItem key={"Access"}>
                                <div className="flex items-center">
                                    <BsShieldLockFill /><span className="ml-2">SSH and Access</span>
                                </div>
                            </ListboxItem>
                            <ListboxItem key={"Emails"}>
                                <div className="flex items-center">
                                    <TfiEmail /><span className="ml-2">Emails</span>
                                </div>
                            </ListboxItem>
                            <ListboxItem key={"Session"}>
                                <div className="flex items-center">
                                    <SiSession /><span className="ml-2">Session</span>
                                </div>
                            </ListboxItem>
                            <ListboxItem key="Secrets">
                                <div className="flex items-center">
                                    <SiAwssecretsmanager /><span className="ml-2">Secrets</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                    </Listbox>
                </ListboxWrapper>
            </div>
            <Card>
                <CardBody>
                    {
                        selectedValue === "Access" && <UserSettingAccess props={props.props}/>
                    }
                </CardBody>
            </Card>
        </div>
    )
}