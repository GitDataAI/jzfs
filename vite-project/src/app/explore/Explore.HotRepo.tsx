import {HotRepo} from "@/types.ts";
import {useEffect, useState} from "react";
import {Avatar, BreadcrumbItem, Breadcrumbs, Card, CardBody, CardHeader} from "@heroui/react";
import {Listbox, ListboxItem} from "@heroui/listbox";
import {createAvatar} from "@dicebear/core";
import {lorelei} from "@dicebear/collection";
import {useNavigate} from "react-router-dom";
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
                                     nav(`/${value.owner}/${model.name}`)
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
                                        <Breadcrumbs key={"bordered"} variant={"light"} className="bordered" separator="/">
                                            <BreadcrumbItem onClick={() => {
                                                nav("/" + value.owner)
                                            }}>{value.owner}</BreadcrumbItem>
                                            <BreadcrumbItem>{model.name}</BreadcrumbItem>
                                        </Breadcrumbs>
                                    </div>
                                    <div style={{
                                        display: "flex",
                                        gap: "10px",
                                    }}>
                                        <FaFire />
                                        {value.complex.toFixed(2)}
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