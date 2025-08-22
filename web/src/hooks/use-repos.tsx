import { toast } from "sonner";
import type { Result } from "@/lib/result";

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

export interface RepoItem {
  repo: RepoModel;
  owner: RepoOwner;
  state: RepoState;
}

interface Paginator {
  page: number;
  page_size: number;
}

export interface ReposRecommendResponse {
  total: number;
  items: RepoItem[];
  page: number;
  page_size: number;
}

export const useRepos = () => {
  return {
    fetchRecommendedRepos: async (paginator: Paginator): Promise<ReposRecommendResponse | null> => {
      try {
        const url = `/api/repo?page=${paginator.page}&page_size=${paginator.page_size}`;
        const response = await fetch(url, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (response.ok) {
          const data: Result<ReposRecommendResponse> = await response.json();
          if (data.code === 200 && data.data) {
            return data.data;
          } else {
            toast.warning(data.msg);
            return null;
          }
        } else {
          const errorText = await response.text();
          toast.error(errorText || "Failed to fetch recommended repositories");
          return null;
        }
      } catch (error) {
        console.error("Failed to fetch recommended repositories:", error);
        toast.error("Failed to fetch recommended repositories");
        return null;
      }
    },
  };
};

export default useRepos;