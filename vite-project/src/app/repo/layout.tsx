import "./module.css"
import {RepoHeader} from "@/app/repo/Repo.Header.tsx";
import {useEffect, useState} from "react";
import {RepoApi} from "@/api/RepoApi.tsx";
import {Repository} from "@/types.ts";
import {useParams} from "react-router-dom";
import {toast} from "@pheralb/toast";
import RepoFile from "@/app/repo/Repo.File.tsx";
import {RepoSetting} from "@/app/repo/Repo.Setting.tsx";

const RepoLayout = () => {
    const [Tab, setTab] = useState("")
    const [Load, setLoad] = useState(false);
    const [RepoInfo, setRepoInfo] = useState<null | Repository>()
    const api = new RepoApi();
    const { owner, repo } = useParams() as { owner: string, repo: string }
    const UpData = () => {
        api.GetInfo(owner, repo)
            .then(res => {
                if (res.status !== 200) {
                    toast.error({ text: '获取仓库信息失败' });
                    return;
                }
                if (res.data){
                    setRepoInfo(JSON.parse(res.data).data);
                    setLoad(true);
                }
            })
    }
    useEffect(() => {
        UpData()
    }, [repo,owner]);
    return (
        <div>
            {
                (Load && RepoInfo ) && (
                    <div className="repo">
                        <RepoHeader setTab={setTab} info={RepoInfo} owner={owner} repo={repo}/>
                        {
                            (Tab === "file" || Tab === "" || Tab === undefined) && <RepoFile info={RepoInfo} owner={owner} repo={repo} upDate={UpData}/>
                        }
                        {
                            Tab === "setting" && <RepoSetting  info={RepoInfo} owner={owner} repo={repo} upDate={UpData}/>
                        }
                    </div>
                )
            }
        </div>
    )
}

export default RepoLayout