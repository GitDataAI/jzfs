import { toast } from "sonner";
import type { Result } from "@/lib/result";
import {createContext} from "react";

export interface UserModel {
    uid: string;
    username: string;
    email: string;
    display_name?: string | null;
    avatar_url?: string | null;
    bio?: string | null;
    location?: string | null;
    website_url?: string | null;
    company?: string | null;
    is_active: boolean;
    is_verified: boolean;
    is_premium: boolean;
    created_at: string;
    updated_at: string;
    last_login_at?: string | null;
    timezone?: string | null;
    language?: string | null;
    theme?: string | null;
    login_count: number;
}

export interface UserData {
    model: UserModel;
    is_owner: boolean;
    is_follow?: boolean;
}

interface RepoModel {
  uid: string;
  namespace: string;
  repo_name: string;
  default_head: string;
  description: string | null;
  is_private: boolean;
  created_at: string;
  updated_at: string;
  storage: string;
}

interface RepoOwner {
  uid: string;
  username: string;
  avatar_url: string;
}

interface RepoState {
  uid: string;
  repo_uid: string;
  stars: number;
  watches: number;
  forks: number;
  created_at: string;
  updated_at: string;
}

interface RepoItem {
  repo: RepoModel;
  owner: RepoOwner;
  state: RepoState;
}

interface Paginator {
  page: number;
  page_size: number;
}

export interface UserReposResponse {
  total: number;
  items: RepoItem[];
  page: number;
  page_size: number;
}

export const UserDataContext = createContext<UserData | null>(null)
export const useUserData = () => {
        return {
            fetchUserData: async (username: string): Promise<UserData | null> => {
                try {
                    const response = await fetch(`/api/users/${username}`, {
                        method: "GET",
                        headers: {
                            "Content-Type": "application/json",
                        },
                    });
                    if (response.ok) {
                        const data: Result<UserData> = await response.json();
                        if (data.code === 200 && data.data) {
                            return data.data;
                        } else {
                            toast.warning(data.msg);
                            return null;
                        }
                    } else {
                        const errorText = await response.text();
                        toast.error(errorText || "Failed to fetch user data");
                        return null;
                    }
                } catch (error) {
                    console.error("Failed to fetch user data:", error);
                    toast.error("Failed to fetch user data");
                    return null;
                }
            },

            fetchUserActive: async (username: string): Promise<UserRepoActiveModel[] | null> => {
                try {
                    const response = await fetch(`/api/users/${username}/active`, {
                        method: "GET",
                        headers: {
                            "Content-Type": "application/json",
                        },
                    });
                    if (response.ok) {
                        const data: Result<UserRepoActiveModel[]> = await response.json();
                        if (data.code === 200 && data.data) {
                            return data.data;
                        } else {
                            toast.warning(data.msg);
                            return null;
                        }
                    } else {
                        const errorText = await response.text();
                        toast.error(errorText || "Failed to fetch user active data");
                        return null;
                    }
                } catch (error) {
                    console.error("Failed to fetch user active data:", error);
                    toast.error("Failed to fetch user active data");
                    return null;
                }
            },

            fetchUserStar: async (username: string, paginator: Paginator): Promise<UserReposResponse | null> => {
                try {
                    const url = `/api/users/${username}/star?page=${paginator.page}&page_size=${paginator.page_size}`;
                    const response = await fetch(url, {
                        method: "GET",
                        headers: {
                            "Content-Type": "application/json",
                        },
                    });
                    if (response.ok) {
                        const data: Result<UserReposResponse> = await response.json();
                        if (data.code === 200 && data.data) {
                            return data.data;
                        } else {
                            toast.warning(data.msg);
                            return null;
                        }
                    } else {
                        const errorText = await response.text();
                        toast.error(errorText || "Failed to fetch user starred repositories");
                        return null;
                    }
                } catch (error) {
                    console.error("Failed to fetch user starred repositories:", error);
                    toast.error("Failed to fetch user starred repositories");
                    return null;
                }
            },

            fetchUserWatch: async (username: string, paginator: Paginator): Promise<UserReposResponse | null> => {
                try {
                    const url = `/api/users/${username}/watch?page=${paginator.page}&page_size=${paginator.page_size}`;
                    const response = await fetch(url, {
                        method: "GET",
                        headers: {
                            "Content-Type": "application/json",
                        },
                    });
                    if (response.ok) {
                        const data: Result<UserReposResponse> = await response.json();
                        if (data.code === 200 && data.data) {
                            return data.data;
                        } else {
                            toast.warning(data.msg);
                            return null;
                        }
                    } else {
                        const errorText = await response.text();
                        toast.error(errorText || "Failed to fetch user watched repositories");
                        return null;
                    }
                } catch (error) {
                    console.error("Failed to fetch user watched repositories:", error);
                    toast.error("Failed to fetch user watched repositories");
                    return null;
                }
            },

            fetchUserRepos: async (username: string, paginator: Paginator): Promise<UserReposResponse | null> => {
                try {
                    const url = `/api/users/${username}/repo?page=${paginator.page}&page_size=${paginator.page_size}`;
                    const response = await fetch(url, {
                        method: "GET",
                        headers: {
                            "Content-Type": "application/json",
                        },
                    });
                    if (response.ok) {
                        const data: Result<UserReposResponse> = await response.json();
                        if (data.code === 200 && data.data) {
                            return data.data;
                        } else {
                            toast.warning(data.msg);
                            return null;
                        }
                    } else {
                        const errorText = await response.text();
                        toast.error(errorText || "Failed to fetch user repositories");
                        return null;
                    }
                } catch (error) {
                    console.error("Failed to fetch user repositories:", error);
                    toast.error("Failed to fetch user repositories");
                    return null;
                }
            },
        };
};

export interface UserRepoActiveModel {
    uid: string;
    name: string;
    email: string;
    user_uid?: string;
    commit: string;
    repo_uid: string;
    time: number;
    offset: number;
}

export default useUserData;