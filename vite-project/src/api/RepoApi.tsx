import {Http} from "@/api/Http.tsx";

export class RepoApi extends Http {
    async CreateRepo(name: string, description: string, visibility: boolean,auto_init: boolean,readme: boolean, default_branch: string){
        return await this.post<string>('/repo/create', {
            name: name,
            description: description,
            private: visibility,
            auto_init: auto_init,
            readme: readme,
            default_branch: default_branch,
        })
    }
    async GetInfo(owner: string, repo: string){
        return await this.post<string>(`/repo/${owner}/${repo}/info`,{})
    }
    async Bhtc(owner: string, repo: string){
        return await this.post<string>(`/repo/${owner}/${repo}/bhct`,{})
    }
    async Tree(owner: string, repo: string,branch: string,head:string){
        return await this.post<string>(`/repo/${owner}/${repo}/branch/${branch}/${head}/tree`,{})
    }
    async File(owner:string, repo: string,path: string, sha: string) {
        return await this.post<Uint8Array>(`/repo/file`, {
            owner: owner,
            repo: repo,
            paths: path,
            sha: sha
        })
    }
}