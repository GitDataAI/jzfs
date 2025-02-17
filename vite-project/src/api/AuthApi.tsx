import {Http} from "@/api/Http.tsx";


export class AuthApi extends Http {
    async Captcha() {
        return await this.get<string>("/auth/captcha");
    }
    async LoginOut(){
        return await this.post<string>("/auth/logout",{});
    }
    async LoginPasswd(username: string, password: string, captcha: string) {
        return await this.post<string>("/auth/passwd", {
            username: username,
            password: password,
        }, {}, {
            "x-captcha": captcha
        });
    }
    async ApplyUser(username: string, password: string, email: string, captcha: string) {
        return await this.post<string>("/auth/apply", {
            username: username,
            password: password,
            email: email,
        }, {}, {
            "x-captcha": captcha
        });
    }
    async CaptchaEmailSend(email: string) {
        return await this.post<string>("/auth/email_send", {
            email: email,
            code: ""
        });
    }
    async CaptchaEmailCheck(email: string, code: string) {
        return await this.post<string>("/auth/email_check", {
            email: email,
            code: code
        });
    }
    async CheckEmailORUsername(email?: string, username?: string){
        return await this.post<string>("/auth/check", {
            email: email,
            username: username
        });
    }
}