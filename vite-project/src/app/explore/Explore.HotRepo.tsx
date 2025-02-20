import {HotRepo, UserModel} from "@/types.ts";
import {useEffect, useState} from "react";
import {Avatar, Card, CardBody, CardHeader} from "@heroui/react";
import {Listbox, ListboxItem} from "@heroui/listbox";
import {createAvatar} from "@dicebear/core";
import {lorelei} from "@dicebear/collection";
import {useNavigate} from "react-router-dom";
import {UserApi} from "@/api/UserApi.tsx";
import {toast} from "@pheralb/toast";
import {AppWrite} from "@/api/Http.tsx";
import { FaFire } from "react-icons/fa";

interface ExploreHotProps {
    hot: HotRepo[]
}

export const ExploreHotRepo = (props: ExploreHotProps) => {
    const [SortStar, setSortStar] = useState<HotRepo[]>([]);
    const [SortClick, setSortClick] = useState<HotRepo[]>([]);
    const [SortFork, setSortFork] = useState<HotRepo[]>([]);
    const [SortComplex, setSortComplex] = useState<HotRepo[]>([]);
    useEffect(()=>{
        setSortStar(props.hot.sort((a, b) => b.star - a.star))
        setSortClick(props.hot.sort((a, b) => b.click - a.click))
        setSortFork(props.hot.sort((a, b) => b.fork - a.fork))
        setSortComplex(props.hot.sort((a, b) => b.complex - a.complex))
    },[props.hot])
    return(
        <div className="explore-hot-repo">
            <Card className="explore-hot-repo-item">
                <CardHeader>
                    Comprehensive ranking
                </CardHeader>
                <RankList repos={SortComplex}/>
            </Card>
            <Card className="explore-hot-repo-item">
                <CardHeader>
                    Click ranking
                </CardHeader>
                <RankList repos={SortClick}/>
            </Card>
            <Card className="explore-hot-repo-item">
                <CardHeader>
                    Star ranking
                </CardHeader>
                <RankList repos={SortStar}/>
            </Card>
            <Card className="explore-hot-repo-item">
                <CardHeader>
                    Fork ranking
                </CardHeader>
                <RankList repos={SortFork}/>
            </Card>
        </div>

    )
}
const userApi = new UserApi();
const RankList = (props: {repos: HotRepo[]}) => {
    const nav = useNavigate();
    return(
        <CardBody>
            <Listbox>
                {
                    props.repos.map((value) => {
                        const model = value.model;
                        if (!model.avatar){
                            const avatar = createAvatar(lorelei, {
                                seed: model.name,
                            });

                            model.avatar = avatar.toDataUri();
                        }
                        return(
                            <ListboxItem style={{
                                display: "flex",
                                gap: "10px",
                            }}
                                 onPress={()=>{
                                    userApi.InfoByUid(model.owner_id)
                                        .then(res=>{
                                            if (res.status !== 200){
                                                toast.error({
                                                    text: "Owner Err"
                                                })
                                            }
                                            const json: AppWrite<UserModel> = JSON.parse(res.data);
                                            if (json.code === 200 && json.data && res.status) {
                                                nav(`/${json.data.username}/${model.name}`)
                                            }else {
                                                toast.error({
                                                    text: "Owner Err"
                                                })
                                            }
                                        })
                                 }}
                                 className={"explore-hot-repo-item-item"}>
                                <div style={{
                                    display: "flex",
                                    justifyContent: "space-between",
                                    alignItems: "center",
                                    width: "100%",
                                }}>
                                    <div style={{
                                        display: "flex",
                                        gap: "10px",
                                        alignItems: "center",
                                    }}>
                                        <Avatar src={model.avatar} size={"sm"}/>
                                        <span>{model.name}</span>
                                    </div>
                                    <div style={{
                                        display: "flex",
                                        gap: "10px",
                                    }}>
                                        <FaFire />
                                        {value.complex}
                                    </div>
                                </div>
                            </ListboxItem>
                        )
                    })
                }
            </Listbox>
        </CardBody>
    )
}