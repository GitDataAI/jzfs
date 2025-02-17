import {UserDashBored} from "@/types.ts";
import {Avatar, Button, Card, CardBody, Input, Select, SelectItem} from "@heroui/react";
import {CgWebsite} from "react-icons/cg";
import {MdLocationOn, MdOutlineDescription} from "react-icons/md";
import {TbTimezone} from "react-icons/tb";
import {FaLanguage} from "react-icons/fa";
import {useState} from "react";
import {timezone} from "@/timezone.ts";
import {language} from "@/language.ts";
import {location} from "@/location.ts";
import {UserApi} from "@/api/UserApi.tsx";
import {toast} from "@pheralb/toast";

const UserActive = (props: {props: UserDashBored}) => {
    const prop = props.props;
    const [Edit, setEdit] = useState(false)
    const user = new UserApi();
    const [Form, setFrom] = useState({
        description: prop.user.description || "",
        website: prop.user.website || "",
        location: prop.user.location || "",
        timezone: prop.user.timezone || "",
        language: prop.user.language || "English",
    })
    const UpTional = async () => {
        user.UpTional(
            Form.description,
            Form.website,
            Form.location,
            Form.timezone,
            Form.language,
        )
            .then((res) => {
                const json = JSON.parse(res.data);
                if (res.status === 200 && json['code'] === 200 && json['data']) {
                    toast.success({ text: json['message'] });
                    setEdit(false)
                } else {
                    toast.error({ text: json['message'] });
                }
                setEdit(false)
            })
    }

    return (
        <div className="user-active user-bodt">
            <Card>
                <CardBody>
                    <div className="user-active-profile">
                        <Avatar
                            className="w-24 h-24 text-large avatar"
                            src={prop.user.avatar}
                        />
                        <div className={"user-active-profile-info"}>
                    <span style={{
                        fontSize: "1.5rem",
                        fontWeight: "bold"
                    }}>{prop.user.username}</span><br/>
                            <span>{prop.user.name}</span>
                        </div>
                        <div className={"user-active-profile-action"}>
                            <Button onPress={()=>{
                                if (Edit) {
                                    UpTional().then()
                                }
                                setEdit(true)
                            }} className={"button button-primary"} color={"secondary"}>
                                {
                                    Edit ? "Save" : "Edit Profile"
                                }
                            </Button>&nbsp;&nbsp;&nbsp;
                            {
                                Edit && (
                                    <Button onPress={()=>{
                                        setEdit(false)
                                    }}>
                                        Cancel
                                    </Button>
                                )
                            }
                        </div>
                        {
                            !Edit ? (
                                <ul className={"user-active-profile-infos"}>
                                    {
                                        prop.user.description && (
                                            <li>
                                                <span><MdOutlineDescription /><a>{prop.user.description}</a></span>
                                            </li>
                                        )
                                    }
                                    {
                                        prop.user.website && (
                                            <li>
                                                <span><CgWebsite /><a>{prop.user.website}</a></span>
                                            </li>
                                        )
                                    }
                                    {
                                        prop.user.location && (
                                            <li>
                                                <span><MdLocationOn /><a>{prop.user.location}</a></span>
                                            </li>
                                        )
                                    }

                                    {
                                        prop.user.timezone && (
                                            <li>
                                                <span><TbTimezone /><a>{prop.user.timezone}</a></span>
                                            </li>
                                        )
                                    }
                                    {
                                        prop.user.language && (
                                            <li>
                                                <span><FaLanguage /><a>{prop.user.language}</a></span>
                                            </li>
                                        )
                                    }
                                </ul>
                            ):(
                                <ul className={"user-active-profile-infos"}>
                                    <li>
                                        <Input onChange={(e)=>{
                                            setFrom({
                                                ...Form,
                                                description: e.target.value
                                            })
                                        }} value={Form.description} type="text" placeholder="Description" />
                                    </li>
                                    <li>
                                        <Input onChange={(e)=>{
                                            setFrom({
                                                ...Form,
                                                website: e.target.value
                                            })
                                        }} value={Form.website} type="text" placeholder="Website" />
                                    </li>
                                    <li>
                                        <Select onChange={(e)=>{
                                            setFrom({
                                                ...Form,
                                                location: e.target.value
                                            })
                                        }} isRequired defaultSelectedKeys={[Form.location]}>
                                            {
                                                location.map((value) => {
                                                    return (
                                                        <SelectItem key={value} value={value}>
                                                            {value}
                                                        </SelectItem>
                                                    )
                                                })
                                            }
                                        </Select>
                                    </li>
                                    <li>
                                        <Select onChange={(e)=>{
                                            setFrom({
                                                ...Form,
                                                timezone: e.target.value
                                            })
                                        }} isRequired defaultSelectedKeys={[Form.timezone]}>
                                            {
                                                timezone.map((value) => {
                                                    return (
                                                        <SelectItem key={value.value} value={value.value}>
                                                            {value.label}
                                                        </SelectItem>
                                                    )
                                                })
                                            }
                                        </Select>
                                    </li>
                                    <li>
                                        <Select onChange={(e)=>{
                                            setFrom({
                                                ...Form,
                                                language: e.target.value
                                            })
                                        }} isRequired defaultSelectedKeys={[Form.language]}>
                                            {
                                                language.map((value) => {
                                                    return (
                                                        <SelectItem key={value} value={value}>
                                                            {value}
                                                        </SelectItem>
                                                    )
                                                })
                                            }
                                        </Select>
                                    </li>
                                </ul>
                            )
                        }

                    </div>
                </CardBody>
            </Card>
            <Card style={{
                marginLeft: "1rem"
            }}>
                <CardBody>
                    <div className="user-active-active">
                    </div>
                </CardBody>
            </Card>
        </div>
    )
}

export default UserActive