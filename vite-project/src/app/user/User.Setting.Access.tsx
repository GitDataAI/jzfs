import {SSHKeyModel, TokenCreateReopens, TokenModel, UserDashBored} from "@/types.ts";
import {Button, Card, CardBody, Divider, Form, Input, Radio, RadioGroup, Snippet} from "@heroui/react";
import {UserApi} from "@/api/UserApi.tsx";
import {useEffect, useState} from "react";
import {Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, useDisclosure} from "@heroui/modal";
import {AppWrite} from "@/api/Http.tsx";
import {toast} from "@pheralb/toast";
import {Textarea} from "@heroui/input";


const api = new UserApi();

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const UserSettingAccess = (_props: { props: UserDashBored }) => {
    const [TokenList, setTokenList] = useState<TokenModel[]>([]);
    const [SSHList, setSSHList] = useState<SSHKeyModel[]>([]);
    const TokenModal = useDisclosure();
    const SSHModal = useDisclosure();
    const TokenResModel = useDisclosure();
    const [Res, SetRes] = useState<TokenCreateReopens | undefined>(undefined);

    useEffect(() => {
        api.TokenList()
            .then(res => {
                if (res.status === 200) {
                    const json: AppWrite<TokenModel[]> = JSON.parse(res.data);
                    if (json.code === 200 && json.data) {
                        setTokenList(json.data);
                    }
                }
            })
        api
            .SSHList()
            .then(res=>{
                if (res.status === 200) {
                    const json: AppWrite<SSHKeyModel[]> = JSON.parse(res.data);
                    if (json.code === 200 && json.data) {
                        setSSHList(json.data);
                    }
                }
            })
    }, []);
    return (
        <div className="user-access">
            <div className="user-access-ssh">
                <div className="user-access-title">
                    <h1>
                        SSH keys
                    </h1>
                    <Button onPress={SSHModal.onOpen}>New SSH key</Button>
                </div>
                <Divider/>
                <span>
                    This is a list of SSH keys associated with your account. Remove any keys that you do not recognize.
                </span>
                {
                    SSHList.length === 0 && (
                        <EmptyAccess props={"SSH"}/>
                    )
                }
                {
                    SSHList.map((item, index) => {
                        return (
                            <SSHItem key={index} props={item}/>
                        )
                    })
                }
                <br/>
            </div>
            <div className="user-access-token">
                <div className="user-access-title">
                    <h1>
                        Personal access tokens
                    </h1>
                    <Button onPress={TokenModal.onOpen}>New Token</Button>
                </div>
                <Divider/>
                <span>
                        Tokens you have generated that can be used to access the GitData.
                </span>
                {
                    TokenList.length === 0 && (
                        <EmptyAccess props={"Token"}/>
                    )
                }
                {
                    TokenList.map((item, index) => {
                        return (
                            <TokenItem key={index} props={item}/>
                        )
                    })
                }
            </div>
            <div style={{
                position: "fixed",
                zIndex: "9999999"
            }}>
                <Modal
                    onOpenChange={TokenModal.onOpenChange}
                    backdrop="blur"
                    isOpen={TokenModal.isOpen}
                    size={"2xl"}
                    onClose={TokenModal.onClose}
                >
                    <ModalContent>
                        <ModalHeader>
                            New Personal access tokens
                        </ModalHeader>
                        <NewToken setRes={SetRes} onClose={TokenModal.onClose} openProduct={TokenResModel.onOpen}/>
                    </ModalContent>
                </Modal>
                <Modal
                    onOpenChange={SSHModal.onOpenChange}
                    backdrop="blur"
                    isOpen={SSHModal.isOpen}
                    size={"2xl"}
                    onClose={SSHModal.onClose}
                >
                    <ModalContent>
                        <ModalHeader>
                            New Personal access tokens
                        </ModalHeader>
                        <NewSSH onClose={SSHModal.onClose}/>
                    </ModalContent>
                </Modal>
                {
                    Res && (
                        <Modal
                            onOpenChange={TokenResModel.onOpenChange}
                            isOpen={TokenResModel.isOpen}
                            size={"2xl"}
                            onClose={TokenResModel.onClose}
                        >
                            <ModalContent>
                                <Card>
                                    <CardBody>
                                        <span>This token will only be displayed once, please keep it safe afterwards</span>
                                        <div style={{
                                            marginTop: "20px",
                                            marginBottom: "10px",
                                            justifyContent: "center",
                                            alignItems: "center",
                                            display: "flex",
                                            gap: "10px",
                                        }}>
                                            <Snippet size="sm">{Res.token}</Snippet>
                                        </div>
                                    </CardBody>
                                </Card>
                            </ModalContent>
                        </Modal>
                    )
                }
            </div>
        </div>
    )
}

const TokenItem = (props: { props: TokenModel }) => {
    const last_use = props.props.use_history ? props.props.use_history[0] : undefined;
    return (
        <Card className="user-access-token-item">
            <CardBody style={{
                display: "grid",
                gridTemplateColumns: "1fr 1fr",
            }}>
                <div>
                    <h1>{props.props.name}</h1>
                    <span>{props.props.description}</span><br/>
                    <span>
                        Uid: {props.props.uid}
                    </span><br/>
                    {
                        last_use ? (
                            <>

                            </>
                        ) : (
                            <span style={{
                                color: "red"
                            }}>
                            Never used
                            </span>
                        )
                    }<br/>
                    <span>
                    Created at: {props.props.created_at.toString()}
                </span>
                </div>
                <div style={{
                    display: 'flex',
                    justifyContent: 'flex-end',
                    alignItems: 'center',
                    marginTop: '1rem'
                }}>
                    <Button
                        className="w-2"
                        style={{
                            padding: '0.5rem 1rem',
                            height: 'fit-content'
                        }}
                        onPress={() => {
                            api.TokenDelete({
                                name: props.props.name,
                                uid: props.props.uid
                            }).then(res => {
                                const json: AppWrite<string> = JSON.parse(res.data);
                                if (json.code === 200 && res.status === 200) {
                                    toast.success({
                                        text: "Delete Success"
                                    })
                                } else {
                                    toast.error({
                                        text: "Delete Failed:" + json.msg
                                    })
                                }
                            })
                        }}
                    >
                        Delete
                    </Button>
                </div>
            </CardBody>
        </Card>
    )
}

const SSHItem = (props: { props: SSHKeyModel }) => {
    return (
        <Card className="user-access-token-item">
            <CardBody style={{
                display: "grid",
                gridTemplateColumns: "1fr 1fr",
            }}>
                <div>
                    <h1>{props.props.name}</h1>
                    <span>{props.props.description}</span><br/>
                    <span>
                        Uid: {props.props.uid}
                    </span><br/>
                    <span>
                    Created at: {props.props.created_at.toString()}
                </span>
                </div>
                <div style={{
                    display: 'flex',
                    justifyContent: 'flex-end',
                    alignItems: 'center',
                    marginTop: '1rem'
                }}>
                    <Button
                        className="w-2"
                        style={{
                            padding: '0.5rem 1rem',
                            height: 'fit-content'
                        }}
                        onPress={() => {
                            api.SSHDelete(props.props.uid).then(res => {
                                const json: AppWrite<string> = JSON.parse(res.data);
                                if (json.code === 200 && res.status === 200) {
                                    toast.success({
                                        text: "Delete Success"
                                    })
                                } else {
                                    toast.error({
                                        text: "Delete Failed:" + json.msg
                                    })
                                }
                            })
                        }}
                    >
                        Delete
                    </Button>
                </div>
            </CardBody>
        </Card>
    )
}

const EmptyAccess = (props: { props: "Token" | "SSH" }) => {
    return (
        <div className="user-access-empty">
            <div className="empty-illustration">
                <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M12 2v4m0 12v4m-6-6h4m8 0h4M4 12h16"/>
                </svg>
            </div>
            <h3>No Access Records Found</h3>
            <p className="text-secondary">
                You haven't created any {
                props.props === "Token" ? "personal access tokens" : "SSH keys"
            } yet.
            </p>
        </div>
    )
}

interface NewTokenProps {
    setRes: (value: (((prevState: (TokenCreateReopens | undefined)) => (TokenCreateReopens | undefined)) | TokenCreateReopens | undefined)) => void,
    onClose: () => void,
    openProduct: () => void
}

const NewToken = (props: NewTokenProps) => {
    const [From, setFrom] = useState({
        name: "",
        description: "",
        expire: "",
        access: ""
    })
    const onSubmit = () => {
        api.TokenCreate({
            name: From.name,
            description: From.description,
            expire: parseInt(From.expire),
            access: parseInt(From.access)
        }).then(res => {
            if (res.status === 200) {
                const json: AppWrite<TokenCreateReopens> = JSON.parse(res.data);
                console.log(json.data)
                if (json.code === 200 && json.data) {
                    props.setRes(json.data)
                    props.onClose();
                    props.openProduct();
                }
            }
        })
    }
    return (
        <div>
            <Form
                onSubmit={(e) => {
                    onSubmit()
                    e.preventDefault();
                    e.stopPropagation();
                }}
            >
                <ModalBody style={{
                    width: "100%",
                }}>

                    <Input name="name" isRequired label="Token name"
                           placeholder="Token name" onChange={(e) => {
                        setFrom({
                            ...From,
                            name: e.target.value
                        })
                    }}/>
                    <Input name="description" label="Description" placeholder="Description"
                           onChange={(e) => {
                               setFrom({
                                   ...From,
                                   description: e.target.value
                               })
                           }}/>
                    <Input name="expire" isRequired typeof="number" label="Expiration date"
                           placeholder="Expiration date(days or -1 no-limit)" onChange={(e) => {
                        setFrom({
                            ...From,
                            expire: e.target.value
                        })
                    }} validate={(value) => {
                        if (!/^[0-9]+$/.test(value)) {
                            return "Expiration date must be number";
                        }
                    }}/>
                    <RadioGroup name="access" isRequired label="Select Access" onChange={(e) => {
                        setFrom({
                            ...From,
                            access: e.target.value
                        })
                    }}>
                        <Radio value="1">Read</Radio>
                        <Radio value="2">Write</Radio>
                        <Radio value="3">Read and Write</Radio>
                    </RadioGroup>
                </ModalBody>
                <ModalFooter style={{
                    width: "100%",
                }}>
                    <Button type="submit">Create</Button>
                </ModalFooter>
            </Form>
        </div>
    )
}


interface NewSSHProps {
    onClose: () => void
}

const NewSSH = (props: NewSSHProps) => {
    const [From, setFrom] = useState({
        name: "",
        description: "",
        public_key: ""
    })
    const onSubmit = () => {
        api.SSHCreate({
            name: From.name,
            description: From.description,
            public_key: From.public_key
        }).then(res => {
            if (res.status === 200) {
                const json:AppWrite<string> = JSON.parse(res.data);
                if (json.code === 200) {
                    toast.success({
                        text: "Create Success"
                    })
                    props.onClose();
                } else {
                    toast.error({
                        text: "Create Failed:" + json.msg
                    })
                }
            }else {
                toast.error({
                    text: "Create Failed:" + res.data
                })
            }
        })
    }
    return (
        <div>
            <Form
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    onSubmit()
                }}
            >
                <ModalBody style={{
                    width: "100%",
                }}>

                    <Input name="name" isRequired label="Token name"
                           placeholder="Token name" onChange={(e) => {
                        setFrom({
                            ...From,
                            name: e.target.value
                        })
                    }}/>
                    <Input name="description" label="Description" placeholder="Description"
                           onChange={(e) => {
                               setFrom({
                                   ...From,
                                   description: e.target.value
                               })
                           }}/>
                    <Textarea name="public_key" isRequired typeof="number" label="SSH Pubkey" onChange={(e) => {
                        setFrom({
                            ...From,
                            public_key: e.target.value
                        })
                    }} validate={(value) => {
                        if (value.length < 50) {
                            return "Public key must be at least 50 characters";
                        }
                        return true;
                    }}/>
                </ModalBody>
                <ModalFooter style={{
                    width: "100%",
                }}>
                    <Button type="submit">Create</Button>
                </ModalFooter>
            </Form>
        </div>
    )
}