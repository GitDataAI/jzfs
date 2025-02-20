
import "./module.css"
import ExploreSearch from "@/app/explore/Explore.Search.tsx";
import {useEffect, useState} from "react";
import {ExploreApi} from "@/api/ExploreApi.tsx";
import {HotRepo, HotTimeParma} from "@/types.ts";
import {ExploreHotRepo} from "@/app/explore/Explore.HotRepo.tsx";
import {AppWrite} from "@/api/Http.tsx";
import {toast} from "@pheralb/toast";
import {Tab} from "@heroui/tabs";
import {Card, CardBody, Tabs} from "@heroui/react";


const ExploreLayout = () => {
    const exploreApi = new ExploreApi();
    const date = new Date();
    const [HotRepo, setHotRepo] = useState<HotRepo[]>([])
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const [HotParma, setHotParma] = useState<HotTimeParma>({
        start: {
            years: date.getUTCFullYear(),
            month: date.getUTCMonth() + 1,
            day: date.getUTCDate() - 1
        },
        end: {
            years: date.getUTCFullYear(),
            month: date.getUTCMonth() + 1,
            day: date.getUTCDate()
        },
        limit: 50
    })
    useEffect(() => {
        exploreApi.HotRepo(HotParma)
            .then(res=>{
                if (res.status !== 200) {
                    toast.error({
                        text: "Data Request Failed"
                    })
                }
                const json = JSON.parse(res.data) as AppWrite<HotRepo[]>;
                if (json.code === 200 && json.data){
                    setHotRepo(json.data)
                }
            })
    }, []);
    return(
        <div className="explore">
            <ExploreSearch/>
            <Tabs className="explore-hot-tabs">
                 <Tab title={
                     <>
                        <span>
                            Repository Ranks
                        </span>
                         <div>

                         </div>
                     </>
                 }>
                     <ExploreHotRepo hot={HotRepo}/>
                 </Tab>
                 <Tab title="Users Ranks">
                     <Card className="explore-hot-repo">
                         <CardBody>
                         </CardBody>
                     </Card>
                 </Tab>
            </Tabs>
        </div>
    )
}

export default ExploreLayout