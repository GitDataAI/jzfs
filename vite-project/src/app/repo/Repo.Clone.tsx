import {ModalBody, ModalContent, ModalHeader} from "@heroui/modal";
import {Card, CardBody, Code, Divider, Snippet} from "@heroui/react";
import {Tab, Tabs} from "@heroui/tabs";
import useUser from "@/state/useUser.tsx";

interface RepoCloneProps {
    owner: string,
    repo: string,
    http: string,
    ssh: string
}

export const RepoClone = (props: RepoCloneProps) => {
    const user = useUser();
    return (
        <ModalContent>
            <ModalHeader>
                Clone Repository for &nbsp;<Code size="lg">{props.owner}/{props.repo}</Code>
            </ModalHeader>
            <ModalBody>
                <Divider/>
                <Tabs>
                    <Tab key="http" title="HTTP">
                        <Card>
                            <CardBody>
                                <div className="pl-5 pr-5 ">
                                    <p>Http Protocol</p>
                                    <Snippet>{props.http}</Snippet>
                                </div>
                                <p className="mt-5">Tips:</p>
                                <Divider/>

                                <div className="p-5">
                                    To download the code, please copy the following command to the terminal and
                                    execute<br/>
                                    <Snippet>{"git clone " + props.http}</Snippet>
                                </div>
                                {
                                    user.dash && (
                                        <>
                                            <div className="pl-5 pr-5 pb-5">
                                                To ensure that the identity of the code you submitted is correctly
                                                recognized by GitData, please execute the following command to complete the
                                                configuration<br/>
                                                <Snippet>
                                                    {"git config --global user.name " + user.getUser()?.username}
                                                    {"git config --global user.email " + user.getUser()?.email}
                                                </Snippet>
                                            </div>
                                        </>
                                    )
                                }
                            </CardBody>
                        </Card>
                    </Tab>
                    <Tab key="ssh" title="SSH">
                        <Card>
                            <CardBody>
                                <div className="pl-5 pr-5 ">
                                    <p>SSH Protocol</p>
                                    <Snippet>{props.ssh}</Snippet>
                                </div>
                                <p className="mt-5">Tips:</p>
                                <Divider/>

                                <div className="p-5">
                                    To download the code, please copy the following command to the terminal and
                                    execute<br/>
                                    <Snippet>{"git clone " + props.ssh}</Snippet>
                                </div>
                                {
                                    user.dash && (
                                        <>
                                            <div className="pl-5 pr-5 pb-5">
                                                To ensure that the identity of the code you submitted is correctly
                                                recognized by GitData, please execute the following command to complete the
                                                configuration<br/>
                                                <Snippet>
                                                    {"git config --global user.name " + user.getUser()?.username}
                                                    {"git config --global user.email " + user.getUser()?.email}
                                                </Snippet>
                                            </div>
                                        </>
                                    )
                                }
                            </CardBody>
                        </Card>
                    </Tab>
                </Tabs>
            </ModalBody>
        </ModalContent>
    );
};