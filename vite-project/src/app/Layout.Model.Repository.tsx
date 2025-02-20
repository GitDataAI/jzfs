import {ModalBody, ModalContent, ModalFooter, ModalHeader} from "@heroui/modal";
import {
    Button,
    Checkbox,
    CheckboxGroup,
    Divider,
    Form,
    Input,
    Radio,
    RadioGroup,
    Select,
    SelectItem,
} from "@heroui/react";
import {RepoApi} from "@/api/RepoApi.tsx";
import {toast} from "@pheralb/toast";
import {useEffect, useState} from "react";
import {RepoAccess} from "@/types.ts";
import useUser from "@/state/useUser.tsx";


interface LayoutModelRepositoryProps {
    onClose: () => void
}

const LayoutModelRepository = (props: LayoutModelRepositoryProps) => {
    const repo = new RepoApi();
    const [Access, setAccess] = useState<RepoAccess[]>([])
    const user = useUser();
    useEffect(() => {
        setAccess([])
        repo.Access().then(res=>{
            if (res.status === 200 && res.data){
                const json = JSON.parse(res.data);
                const data:RepoAccess[] = json.data;
                console.log(data)
                for (let i = 0; i < data.length; i++) {
                    console.log(data[i])
                    setAccess((pre)=>[...pre, data[i]])
                }
            }
        })
    }, []);
    const CreateRepo = async (action: string) => {
        const json = JSON.parse(action)
        const payload = {
            name: json['name'],
            description: json['description'],
            visibility: json['visibility'] !== 'Public',
            auto_init: true,
            readme: json['readme'] === "true",
            default_branch: json['default_branch'],
            owner: json['owner'],
        };
        console.log(payload);
        if (!payload.owner || payload.owner === "") {
            toast.error({
                text: "Owner is required",
            })
            return;
        }

        const res = await repo
            .CreateRepo(
                payload.name,
                payload.description,
                payload.visibility,
                payload.auto_init,
                payload.readme,
                payload.default_branch,
                payload.owner
            );
        const jsonb = JSON.parse(res.data);
        if (res.status === 200 && jsonb['code'] === 200) {
            toast.success({
                text: jsonb['msg'],
            })
            props.onClose()
            window.location.reload();
        } else {
            toast.error({
                text: jsonb['msg'],
            })
        }
    }
    return (
        <ModalContent>
            {(onClose) => (
                <>
                    <ModalHeader className="flex flex-col gap-1">Create Repository</ModalHeader>
                    <Form
                        id="LayoutModelRepository"
                        validationBehavior="native"
                        onSubmit={(e) => {
                            e.preventDefault();
                            const data = Object.fromEntries(new FormData(e.currentTarget));
                            CreateRepo(JSON.stringify(data)).then();
                        }}
                    >
                        <ModalBody
                            style={{
                                display: "flex",
                                flexDirection: "column",
                                gap: "1rem",
                                width: "100%",
                            }}
                        >
                            <Select
                                isRequired
                                defaultSelectedKeys={[user.dash!.user.uid]}
                                name={"owner"}
                                labelPlacement="outside"
                                label="Owner"
                                title="Select Owner">
                                {
                                    Access.map((item) => {
                                        return (
                                            <SelectItem style={{ display: "flex" }} key={item.owner_uid}  itemID={item.owner_uid}>
                                                {item.name}
                                            </SelectItem>

                                        );
                                    })
                                }
                            </Select>
                            <Input
                                isRequired
                                errorMessage="Repository name must be between 2 and 100 characters"
                                label="Name"
                                labelPlacement="outside"
                                name="name"
                                placeholder="Enter repository name"
                                type="tel"
                                validate={(value) => {
                                    const isValid = value.length >= 2 && value.length <= 100;
                                    if (!isValid) {
                                        return "Repository name must be between 2 and 100 characters";
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
                            <CheckboxGroup
                                color="success"
                                name="readme"
                                label="Initialize this repository with">
                                <Checkbox value="true">
                                    Add a README file
                                </Checkbox>
                            </CheckboxGroup>
                            <Divider/>
                            <Input
                                errorMessage="Please enter a valid default branches"
                                label="Default Branch"
                                labelPlacement="outside"
                                name="default_branch"
                                placeholder="Enter repository default branch"
                                type="tel"
                                defaultValue="main"
                            />
                        </ModalBody>
                        <ModalFooter style={{
                            display: "flex",
                            gap: "1rem",
                        }}>
                            <Button color="danger" variant="light" type="button" onPress={onClose}>
                                Close
                            </Button>
                            <Button color="primary" variant="flat" type="reset">
                                Reset
                            </Button>
                            <Button color="primary" type="submit">
                                Create
                            </Button>
                        </ModalFooter>
                    </Form>
                </>
            )}
        </ModalContent>
    )
}

export default LayoutModelRepository