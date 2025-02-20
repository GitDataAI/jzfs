import {Repository} from "@/types.ts";
import {Card, CardBody} from "@heroui/react";
import {ListboxWrapper} from "@/app/user/User.Repository.tsx";
import {Listbox, ListboxItem, ListboxSection} from "@heroui/listbox";
import React, {useEffect} from "react";
import {MdOutlineViewAgenda} from "react-icons/md";
import {FiUsers} from "react-icons/fi";
import {GoCodeReview, GoGitBranch} from "react-icons/go";
import {FaLaptopCode, FaTags} from "react-icons/fa";
import {SiAwssecretsmanager} from "react-icons/si";

interface RepoSettingProps {
    info: Repository,
    owner: string,
    repo: string,
    upDate: () => void
}


export const RepoSetting = (props:RepoSettingProps) => {
    const [selectedKeys, setSelectedKeys] = React.useState(['General']);
    console.log(props.owner)
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
                            <ListboxItem key={"General"}>
                                <div className="flex items-center">
                                    <MdOutlineViewAgenda /><span className="ml-2">General</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection title="access">
                            <ListboxItem key={"Collaborators"}>
                                <div className="flex items-center">
                                    <FiUsers /><span className="ml-2">Collaborators</span>
                                </div>
                            </ListboxItem>
                            <ListboxItem key={"Moderation"}>
                                <div className="flex items-center">
                                    <GoCodeReview /><span className="ml-2">Moderation and Review</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection title="Repository">
                            <ListboxItem key={"Branches"}>
                                <div className="flex items-center">
                                    <GoGitBranch /><span className="ml-2">Branches</span>
                                </div>
                            </ListboxItem>
                            <ListboxItem key={"Tags"}>
                                <div className="flex items-center">
                                    <FaTags /><span className="ml-2">Tags</span>
                                </div>
                            </ListboxItem>
                        </ListboxSection>
                        <ListboxSection title="Automation">
                            <ListboxItem key="Runner">
                                <div className="flex items-center">
                                    <FaLaptopCode /><span className="ml-2">Runner</span>
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
                <CardBody></CardBody>
            </Card>
        </div>
    )
}