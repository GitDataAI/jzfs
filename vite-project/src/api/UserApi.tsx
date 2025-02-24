import {AppWrite, Http} from "@/api/Http.tsx";
import {TokenCreate, TokenCreateReopens, TokenDelete} from "@/types.ts";

export class UserApi extends Http {
    async GetNow() {
        return await this.get<string>('/user/now');
    }
    async DashBoredData(username: string) {
        return await this.get<string>('/user/'+username+'/dashbored')
    }
    async UpTional(
        description?: string,
        website?: string,
        location?: string,
        timezone?: string,
        language?: string,
    ) {
        return await this.patch<string>('/user/uptional', {
            description: description,
            website: website,
            location: location,
            timezone: timezone,
            language: language,
        })
    }
    async InfoByUid(uid: string) {
        return await this.post<string>('/user/uid/'+uid,{})
    }
    async TokenList(){
        return await this.patch<string>('/user/token',{})
    }
    async TokenCreate(parma: TokenCreate){
        return await this.post<AppWrite<TokenCreateReopens>>('/user/token',parma)
    }
    async TokenDelete(parma: TokenDelete){
        return await this.put<AppWrite<string>>('/user/token',parma)
    }
}