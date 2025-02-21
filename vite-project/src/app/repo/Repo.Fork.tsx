import {ModalBody} from "@heroui/modal";
import {useEffect, useState} from "react";
import {RepoAccess} from "@/types.ts";
import {RepoApi} from "@/api/RepoApi.tsx";
import useUser from "@/state/useUser.tsx";
import {Button, Divider, Form, Input, Radio, RadioGroup, Select, SelectItem} from "@heroui/react";
import {toast} from "@pheralb/toast";
import {useNavigate} from "react-router-dom";

export interface RepoForkProps {
    owner: string,
    repo: string,
    close: () => void
}

export const RepoFork = (props: RepoForkProps) => {
    const repo = new RepoApi();
    const [Access, setAccess] = useState<RepoAccess[]>([])
    const nav = useNavigate();
    const user = useUser();
    useEffect(() => {
        setAccess([])
        repo.Access().then(res => {
            if (res.status === 200 && res.data) {
                const json = JSON.parse(res.data);
                const data: RepoAccess[] = json.data;
                for (let i = 0; i < data.length; i++) {
                    console.log(data[i])
                    setAccess((pre) => [...pre, data[i]])
                }
            }
        })
    }, []);
    const [Owner, setOwner] = useState(user.dash!.user.username);
    const Fork = (payload: {
        owner: string,
        name: string,
        description: string,
        visibility: boolean,
    }) => {
        repo.Fork(
            props.owner,
            props.repo,
            payload.owner,
            payload.name,
            payload.visibility,
            payload.description
        )
            .then(res => {
                if (res.status === 200 && res.data) {
                    const json = JSON.parse(res.data);
                    if (json['code'] === 200) {
                        toast.success({
                            text: json['msg'],
                        })
                        props.close();
                        nav("/" + Owner + "/" + payload.name)
                    } else {
                        toast.error({
                            text: json['msg'],
                        })
                    }
                }
            })
    }
    return (
        <ModalBody>
            <Form
                id="LayoutModelRepositoryFork"
                validationBehavior="native"
                onSubmit={(e) => {
                    e.preventDefault();
                    const data = Object.fromEntries(new FormData(e.currentTarget));
                    const payload = {
                        owner: data['owner'].toString(),
                        name: data['name'].toString(),
                        description: data['description'].toString(),
                        visibility: data['visibility'] === "Public",
                    }
                    Fork(payload)
                }}
            >
                <Select
                    isRequired
                    defaultSelectedKeys={[user.dash!.user.uid]}
                    name={"owner"}
                    labelPlacement="outside"
                    label="Owner"
                    onSelectionChange={(pr) => {
                        const d = Access.find((x) => x.owner_uid === pr.currentKey);
                        if (d) {
                            setOwner(d.name);
                        }
                    }}
                    title="Select Owner">
                    {
                        Access.map((item) => {
                            return (
                                <SelectItem style={{
                                    display: "flex"
                                }} key={item.owner_uid} itemID={item.owner_uid}>
                                    {item.name}
                                </SelectItem>
                            );
                        })
                    }
                </Select>
                <Input
                    isRequired
                    errorMessage={(v) => {
                        if (v.isInvalid) {
                            return v.validationErrors[0].toString()
                        }
                    }}
                    label="Name"
                    labelPlacement="outside"
                    name="name"
                    placeholder="Enter repository name"
                    type="tel"
                    defaultValue={props.repo}
                    aria-autocomplete={"none"}
                    validate={(value) => {
                        const isValid = value.length >= 2 && value.length <= 100;
                        if (!isValid) {
                            return "you can use the name";
                        }
                        const owner = Access.find((x) => x.name === Owner);
                        if (owner) {
                            if (owner.repos.includes(value)) {
                                return "Repository name already exists";
                            }
                        }
                        return true;
                    }}
                />

                <Input
                    errorMessage="Please enter a valid email"
                    label="Description(optional)"
                    labelPlacement="outside"
                    name="description"
                    placeholder="Enter repository descrition"
                    type="tel"
                />
                <Divider/>
                <RadioGroup
                    isRequired
                    name="visibility"
                    color="success"
                    label="Select the visibility of the repository">
                    <Radio
                        description="Anyone on the internet can see this repository. You choose who can commit."
                        value="Public">
                        Public
                    </Radio>
                    <Radio description="You choose who can see and commit to this repository."
                           value="Private">
                        Private
                    </Radio>
                </RadioGroup>
                <Divider/>
                <div style={{
                    display: "flex",
                    justifyContent: "flex-end",
                    alignItems: "center",
                    gap: "1rem",
                    marginRight: "auto"
                }}>
                    <Button color="danger" variant="light" type="button" onPress={props.close}>
                        Close
                    </Button>
                    <Button color="primary" variant="flat" type="reset">
                        Reset
                    </Button>
                    <Button color="primary" type="submit">
                        Create
                    </Button>
                </div>
            </Form>
        </ModalBody>
    )
}