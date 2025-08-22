import { toast } from "sonner";
import type { Result } from "@/lib/result";
import {createContext} from "react";

export interface RepoModel {
  uid: string;
  repo_name: string;
  namespace: string;
  description: string | null;
  is_private: boolean;
  created_at: string;
  updated_at: string;
  default_head: string;
  storage: string;
}

export interface RepoOwner {
  uid: string;
  username: string;
  display_name: string | null;
  avatar: string | null;
  team: boolean;
}

export interface RepoData {
  model: RepoModel;
  owner: RepoOwner;
  is_owner: boolean;
}

export interface GitRefs {
  uid: string;
  repo_uid: string;
  ref_name: string;
  ref_git_id: string;
  ref_type: string;
  created_at: string;
  updated_at: string;
}

export interface BranchInfo {
  branch: GitRefs;
  head: string | object;
}

export interface CommitAuthor {
  name: string;
  email: string;
  avatar: string | null;
}


export interface CommitInfo {
  uid: string;
  message: string;
  commiter: CommitAuthor;
  time: number;
  author: CommitAuthor;
  parents: string[];
  tree: string;
  commit_id: string;
}

export interface CommitData {
  data: CommitInfo[],
  total: number
}

 export type TreeKind = 'Tree' | 'Blob';

export interface TreeItem {
  path: string;
  kind: TreeKind;
  name: string;
}

export interface TreeItemLastCommit {
  item: TreeItem;
  commit_oid: string;
  commit_message: string;
  commit_time: number;
  commit_offset: number;
}

export const RepoDataContext = createContext<RepoData | undefined>(undefined);
export const useRepoData = () => {
  return {
    fetchRepoData: async (namespace: string, repoName: string): Promise<RepoData | null> => {
      try {
        const response = await fetch(`/api/repo/${namespace}/${repoName}`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (response.ok) {
          const data: Result<RepoData> = await response.json();
          if (data.code === 200 && data.data) {
            return data.data;
          } else {
            toast.warning(data.msg);
            return null;
          }
        } else {
          const errorText = await response.text();
          toast.error(errorText || "Failed to fetch repository data");
          return null;
        }
      } catch (error) {
        console.error("Failed to fetch repository data:", error);
        toast.error("Failed to fetch repository data");
        return null;
      }
    },

    fetchRepoBranches: async (namespace: string, repoName: string): Promise<BranchInfo[] | null> => {
      try {
        const response = await fetch(`/api/repo/${namespace}/${repoName}/refs`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (response.ok) {
          const data: Result<BranchInfo[]> = await response.json();
          if (data.code === 200 && data.data) {
            return data.data;
          } else {
            toast.warning(data.msg);
            return null;
          }
        } else {
          const errorText = await response.text();
          toast.error(errorText || "Failed to fetch repository branches");
          return null;
        }
      } catch (error) {
        console.error("Failed to fetch repository branches:", error);
        toast.error("Failed to fetch repository branches");
        return null;
      }
    },

    fetchRepoCommits: async (namespace: string, repoName: string, branch: string, page: number, pageSize: number): Promise<CommitData | null> => {
      try {
        const response = await fetch(`/api/repo/${namespace}/${repoName}/commit/${branch}?page=${page}&page_size=${pageSize}`, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (response.ok) {
          const data: Result<CommitData> = await response.json();
          if (data.code === 200 && data.data) {
            return data.data;
          } else {
            toast.warning(data.msg);
            return null;
          }
        } else {
          const errorText = await response.text();
          toast.error(errorText || "Failed to fetch repository commits");
          return null;
        }
      } catch (error) {
        console.error("Failed to fetch repository commits:", error);
        toast.error("Failed to fetch repository commits");
        return null;
      }
    },

    fetchRepoTree: async (namespace: string, repoName: string, refs: string, treeOid?: string, dir: string = ""): Promise<TreeItemLastCommit[] | null> => {
      try {
        let url = `/api/repo/${namespace}/${repoName}/tree/${refs}/${dir}`;
        if (treeOid) {
          url += `&tree_oid=${encodeURIComponent(treeOid)}`;
        }

        const response = await fetch(url, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (response.ok) {
          const data: Result<TreeItemLastCommit[]> = await response.json();
          if (data.code === 200 && data.data) {
            return data.data;
          } else {
            toast.warning(data.msg);
            return null;
          }
        } else {
          const errorText = await response.text();
          toast.error(errorText || "Failed to fetch repository tree");
          return null;
        }
      } catch (error) {
        console.error("Failed to fetch repository tree:", error);
        toast.error("Failed to fetch repository tree");
        return null;
      }
    },
  };
};

export default useRepoData;