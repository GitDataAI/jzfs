import {Http} from "@/api/Http.tsx";

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
}