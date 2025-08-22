import { useState } from "react";
import type { Result } from "@/lib/result.tsx";
import { toast } from "sonner";

interface AccessKey {
  id: string;
  title: string;
  description?: string;
  expiration: string;
  created_at: string;
  last_used?: string;
  repo_access: number;
  email_access: number;
  event_access: number;
  follow_access: number;
  gpg_access: number;
  ssh_access: number;
  webhook_access: number;
  wiki_access: number;
  project_access: number;
  issue_access: number;
  comment_access: number;
  profile_access: number;
}

interface UseAccessKeysImpl {
  accessKeys: AccessKey[];
  isLoading: boolean;
  currentPage: number;
  pageSize: number;
  totalCount: number;
  fetchAccessKeys: (page?: number, pageSize?: number) => Promise<void>;
  createAccessKey: (key: Partial<AccessKey>) => Promise<void>;
  deleteAccessKey: (id: string) => Promise<void>;
}

const useAccessKeys = (): UseAccessKeysImpl => {
  const [accessKeys, setAccessKeys] = useState<AccessKey[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [pageSize, setPageSize] = useState<number>(10);
  const [totalCount, setTotalCount] = useState<number>(0);

  const fetchAccessKeys = async (page = 1, pageSize = 10) => {
    setIsLoading(true);
    setCurrentPage(page);
    setPageSize(pageSize);
    try {
      const response = await fetch(`/api/user/setting/access-key?page=${page}&page_size=${pageSize}`, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (response.ok) {
        const data: Result<{ items: AccessKey[], total_count: number }> = await response.json();
        if (data.data && data.code === 200) {
          setAccessKeys(data.data.items);
          setTotalCount(data.data.total_count);
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to fetch access keys:", error);
      toast.error("Failed to fetch access keys");
    } finally {
      setIsLoading(false);
    }
  };

  const createAccessKey = async (key: Partial<AccessKey>) => {
    setIsLoading(true);
    try {
      const response = await fetch("/api/user/setting/access-key", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(key),
      });

      if (response.ok) {
        const data: Result<AccessKey> = await response.json();
        if (data.data && data.code === 200) {
          if (data.data) {
            setAccessKeys(prevKeys => data.data ? [...prevKeys, data.data] : prevKeys);
          }
          toast.success("Access key created successfully");
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to create access key:", error);
      toast.error("Failed to create access key");
    } finally {
      setIsLoading(false);
    }
  };

  const deleteAccessKey = async (id: string) => {
    setIsLoading(true);
    try {
      const response = await fetch(`/api/user/setting/access-key/${id}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (response.ok) {
        const data: Result<undefined> = await response.json();
        if (data.code === 200) {
          setAccessKeys(prevKeys => prevKeys.filter(key => key.id !== id));
          toast.success("Access key deleted successfully");
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to delete access key:", error);
      toast.error("Failed to delete access key");
    } finally {
      setIsLoading(false);
    }
  };

  return {
    accessKeys,
    isLoading,
    currentPage,
    pageSize,
    totalCount,
    fetchAccessKeys,
    createAccessKey,
    deleteAccessKey,
  };
};

export default useAccessKeys;