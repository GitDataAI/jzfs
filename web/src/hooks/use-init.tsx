import type {Result} from "@/lib/result.tsx";
import {toast} from "sonner";

export interface RepoInitParam {
    owner_uid: string,
    repo_name: string,
    repo_description: string,
    repo_is_private: boolean,
    repo_default_branch: string,
}
export interface RepoOwnerSelectItem {
    uid: string,
    username: string,
    display_name?: string,
    avatar?: string,
    team: boolean
}

export interface RepoInitBefore {
    owner_uid: string,
    team: boolean,
    repo_name: string,
}

export interface RepoInitStorage {
    name: string,
    path: string,
    storage_type?: string
}

export const useInit = () => {
    return {
        InitRepoOwnerSelect: async ():Promise<RepoOwnerSelectItem[]> => {
            const res = await fetch(`/api/repo/init/owner`);
            const json:Result<RepoOwnerSelectItem[]> = await res.json();
            if (json.data) {
                return json.data;
            } else {
                toast.error(json.msg);
                return []
            }
        },
        InitRepoBefore: async (param: RepoInitBefore) => {
            const res = await fetch(`/api/repo/init`, {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(param)
            });
            const json:Result<RepoInitBefore> = await res.json();
            if (json.code === 200) {
                return "";
            } else {
                return json.msg;
            }
        },
        InitRepoStorage: async ():Promise<RepoInitStorage[] | string> => {
            const res = await fetch(`/api/repo/init/storage`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                },
            });
            const json:Result<RepoInitStorage[]> = await res.json();
            if (json.code === 200 && json.data) {
                return json.data;
            } else {
                return json.msg;
            }
        },
        InitRepo: async (param:RepoInitParam) => {
            const res = await fetch(`/api/repo/init`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(param)
            });
            const json:Result<RepoInitParam> = await res.json();
            if (json.code === 200) {
                return "";
            } else {
                return json.msg;
            }
        },
        InitTeam: async () => {

        },
        InitProject: async () => {

        },
        InitDataSet: async () => {

        },
    }
}