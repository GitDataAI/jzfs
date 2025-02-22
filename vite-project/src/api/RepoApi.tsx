import {Http} from "@/api/Http.tsx";

export class RepoApi extends Http {
    async CreateRepo(name: string, description: string, visibility: boolean,auto_init: boolean,readme: boolean, default_branch: string, owner: string){
        return await this.post<string>('/repo/create', {
            name: name,
            description: description,
            private: visibility,
            auto_init: auto_init,
            readme: readme,
            default_branch: default_branch,
            owner: owner
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
    async Access(){
        return await this.get<string>('/repo/access')
    }
    async Star(owner: string, repo: string){
        return await this.post<string>(`/repo/${owner}/${repo}/star`,{})
    }
    async Watch(owner: string, repo: string, level: number){
        return await this.post<string>(`/repo/${owner}/${repo}/watch/${level}`,{})
    }
    async Fork(owner: string, repo: string, owner_uid: string, name: string,prv:boolean, description?: string){
        return await this.post<string>(`/repo/${owner}/${repo}/fork`,{
            owner_uid: owner_uid,
            name: name,
            description: description,
            private: prv
        })
    }
    async OneCommit(owner: string, repo: string, branch: string, sha: string){
        return await this.get<string>(`/repo/${owner}/${repo}/branch/${branch}/sha/${sha}`)
    }
}