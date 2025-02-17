export interface AppWrite<T> {
    code: number;
    msg: string;
    data?: T;
}

import {
    Axios,
    AxiosError,
    AxiosHeaders,
    AxiosRequestConfig,
    AxiosResponse,
    Method,
    RawAxiosRequestHeaders
} from 'axios';

export function api(): string {
    return '/api';
}

export class Http {
    public axios: Axios = new Axios({
        baseURL: api(),
    });

    async get<T>(url: string, parma?: {[key: string]: string}, header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.get(url, {
            params: parma,
            headers: header
        });
    }
    async post<T>(url: string, data: object, parma?: {[key: string]: string}, header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.post(url, JSON.stringify(data), {
            params: parma,
            headers: {
                "content-type": "application/json",
                ...header
            }
        });
    }
    async put<T>(url: string, data: object, parma?: {[key: string]: string},header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.put(url,  JSON.stringify(data), {
            params: parma,
            headers: {
                "content-type": "application/json",
                ...header
            }
        });
    }
    async delete<T>(url: string,parma?: {[key: string]: string}, header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.delete(url, {
            params: parma,
            headers: header
        });
    }
    async patch<T>(url: string, data: object,parma?: {key: string, value: string}[], header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.patch(url,  JSON.stringify(data), {
            params: parma,
            headers: {
                "content-type": "application/json",
                ...header
            }
        });
    }
    async head<T>(url: string,parma?: {key: string, value: string}[], header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.head(url, {
            params: parma,
            headers: header
        });
    }
    async options<T>(url: string,parma?: {key: string, value: string}[], header?: (RawAxiosRequestHeaders & Partial<{
        [Key in Method as Lowercase<Key>]: AxiosHeaders;
    } & {common: AxiosHeaders}>) | AxiosHeaders): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.options(url, {
            params: parma,
            headers: header
        });
    }
    async request<T>(config: AxiosRequestConfig): Promise<AxiosResponse<T, AxiosError>>{
        return await this.axios.request(config);
    }
}