import axios from "axios"
import type {Result} from "@/lib/result.tsx";

export interface EmailCaptchaImpl {
    sendEmailCaptcha: (email: string) => Promise<Result<undefined>>
    verifyEmailCaptcha: (email: string, code: string) => Promise<Result<undefined>>
}



export const useEmailCaptcha:()=>EmailCaptchaImpl = () => {
    return {
        sendEmailCaptcha: async (email: string) => {
            const res = await axios.post(`/api/auth/register/send`, {
                email: email,
                code: ""
            });
            const data:Result<undefined> = res.data;
            return data
        },
        verifyEmailCaptcha: async (email: string, code: string) => {
            const res = await axios.post(`/api/auth/register/verify`, {
                email: email,
                code: code
            });
            const data:Result<undefined> = res.data;
            return data
        }
    }
}