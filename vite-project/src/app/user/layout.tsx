import {useParams, useSearchParams} from "react-router-dom";
import {UserApi} from "@/api/UserApi.tsx";
import {useEffect, useState} from "react";
import {UserDashBored} from "@/types.ts";
import "./module.css"
import {UserHeader} from "@/app/user/User.Header.tsx";
import UserActive from "@/app/user/User.Active.tsx";
import UserRepository from "@/app/user/User.Repository.tsx";
import {UserSetting} from "@/app/user/User.Setting.tsx";


const LayoutUser = () => {
    const {username} = useParams() as { username: string }
    const [Query] = useSearchParams();
    const [Tab, setTab] = useState("active");
    const user = new UserApi();
    const [dashbored, setDashbored] = useState<UserDashBored>()
    const [Load, setLoad] = useState(false)
    useEffect(() => {
        user.DashBoredData(username)
            .then(res => {
                const data: UserDashBored = JSON.parse(res.data).data
                setDashbored(data);
                const page = Query.get("tab");
                if (page) {
                    setTab(page)
                }
                setLoad(true)
            })
    }, [])

    return (
        <>
            {
                (Load && dashbored) && (
                    <div className="user">
                        <UserHeader setTab={setTab} user={dashbored}/>
                        <div className="user-body">
                            {
                                (Tab === "active" || Tab === "" || Tab === undefined) && <UserActive props={dashbored}/>
                            }
                            {
                                Tab === "reposiotry" && <UserRepository props={dashbored}/>
                            }
                            {
                                Tab === "setting" && <UserSetting props={dashbored}/>
                            }
                        </div>
                    </div>
                )
            }
        </>
    )
}


export default LayoutUser