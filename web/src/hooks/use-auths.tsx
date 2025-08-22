import {create} from "zustand";
import {createJSONStorage, persist} from "zustand/middleware";
import type {Result} from "@/lib/result.tsx";
import {toast} from "sonner";

export interface UserSessionImpl {
    user_uid: string;
    username: string;
    email: string;
    display_name?: string;
    avatar_url?: string;
}

export interface UseUserImpl {
    session: UserSessionImpl | null;
    isLoading: boolean;
    isAuthenticated: boolean
    logout: () => Promise<void>;
    login: (username: string, password: string) => Promise<void>;
    register: (username: string, email: string, password: string) => Promise<void>;
    register_after: (username: string, email: string) => Promise<boolean>;
    refresh_context: () => Promise<void>,
}

const useAuths = create<UseUserImpl>()(persist((set) => ({
    session: null,
    isLoading: false,
    isAuthenticated: false,
    async logout() {
        await fetch("/api/auth/logout", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        });
        set({session: null, isAuthenticated: false});
    },
    async login(username: string, password: string) {
        set({isLoading: true});
        const response = await fetch("/api/auth/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                password,
            }),
        });
        if (response.ok) {
            const data:Result<UserSessionImpl> = await response.json();
            if (data.data && data.code === 200) {
                set({session: data.data, isAuthenticated: true});
                window.location.href = "/";
            } else {
                toast.warning(data.msg)
            }
        }
    },
    async register(username: string, email: string, password: string) {
        set({isLoading: true});
        const response = await fetch("/api/auth/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                email,
                password,
            }),
        });
        if (response.ok) {
            const data:Result<UserSessionImpl> = await response.json();
            if (data.data && data.code === 200) {
                set({session: data.data, isAuthenticated: true});
            } else {
                toast.warning(data.msg)
            }
        }
    },
    async register_after(username: string, email: string) {
        set({isLoading: true});
        const response = await fetch("/api/auth/register/after", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                email,
                password: "",
            }),
        });
        if (response.ok) {
            const data:Result<undefined> = await response.json();
            return data.code === 200;
        }
        return false;
    },
    async refresh_context() {
        set({isLoading: true});
        const response = await fetch("/api/auth/context", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        });
        if (response.ok) {
            const data:Result<UserSessionImpl> = await response.json();
            if (data.data && data.code === 200) {
                set({session: data.data, isAuthenticated: true});
            } else {
                set({session: null, isAuthenticated: false});
            }
        }
    },
}),{
    name: "users",
    storage: createJSONStorage(() => localStorage),
}))

export {
    useAuths
};