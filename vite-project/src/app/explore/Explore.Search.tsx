import {Tab, Tabs} from "@heroui/tabs";
import {Button, Card, CardBody, CardHeader, Checkbox, Input} from "@heroui/react";
import {Textarea} from "@heroui/input";
import {useState} from "react";

const ExploreSearch = () => {
    const [InputState, setInputState] = useState('search');
  return (
    <div className="explore-search">
        <div className="explore-search-body">
            <Card style={{
                width: "100%"
            }}>
                <CardHeader>
                    <Tabs onSelectionChange={(x)=>{setInputState(x.toString())}}>
                        <Tab title="Search" key="search"/>
                        <Tab title="Chat" key="chat"/>
                    </Tabs>
                </CardHeader>
                <CardBody style={{
                    width: "100%"
                }}>
                    {
                        InputState === 'search' && (
                            <div className="explore-search-ins">
                                <div className="explore-search-input">
                                    <Input/>
                                    <Button>Search</Button>
                                </div>
                                <div className="explore-search-checkbox">
                                    <Checkbox defaultSelected color="default">
                                        Marketplace
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Repository
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Code
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Issues
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Pull Request
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        User & Group
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Wiki
                                    </Checkbox>
                                    <Checkbox defaultSelected color="default">
                                        Topic
                                    </Checkbox>
                                </div>
                            </div>
                        )
                    }
                    {
                        InputState === 'chat' && (
                            <>
                                <Textarea disabled/>
                                <div style={{
                                    display: "flex",
                                    gap: "1rem",
                                    marginTop: "1rem"
                                }}>
                                    <Input variant="flat" placeholder="请开始你的对话"/>
                                    <Button variant="faded">Send</Button>
                                </div>
                            </>
                        )
                    }
                </CardBody>
            </Card>

        </div>
    </div>
  )
}

export default ExploreSearch