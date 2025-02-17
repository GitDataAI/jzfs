import {ModalBody, ModalContent, ModalFooter, ModalHeader} from "@heroui/modal";
import {Button, Checkbox, CheckboxGroup, Divider, Form, Input, Radio, RadioGroup} from "@heroui/react";
import {RepoApi} from "@/api/RepoApi.tsx";
import {toast} from "@pheralb/toast";


interface LayoutModelRepositoryProps {
    onClose: () => void
}

const LayoutModelRepository = (props: LayoutModelRepositoryProps) => {
    const repo = new RepoApi();
    const CreateRepo = async (action: string) => {
        const json = JSON.parse(action)
        const payload = {
            name: json['name'],
            description: json['description'],
            visibility: json['visibility'] !== 'Public',
            auto_init: true,
            readme: json['readme'] === "true",
            default_branch: json['default_branch']
        };
        const res = await repo
            .CreateRepo(
                payload.name,
                payload.description,
                payload.visibility,
                payload.auto_init,
                payload.readme,
                payload.default_branch
            );
        const jsonb = JSON.parse(res.data);
        if (res.status === 200 && jsonb['code'] === 200) {
            toast.success({
                text: jsonb['msg'],
            })
            props.onClose()
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