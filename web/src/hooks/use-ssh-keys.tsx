import { useState } from "react";
import type { Result } from "@/lib/result.tsx";
import { toast } from "sonner";

export interface SSHKey {
  uid: string;
  name: string;
  fingerprint: string;
  created_at: string;
  updated_at: string;
}

interface UseSSHKeysImpl {
  sshKeys: SSHKey[];
  isLoading: boolean;
  currentPage: number;
  pageSize: number;
  totalCount: number;
  fetchSSHKeys: (page?: number, pageSize?: number) => Promise<void>;
  addSSHKey: (key: { title: string; key: string }) => Promise<void>;
  deleteSSHKey: (id: string) => Promise<void>;
}

const useSSHKeys = (): UseSSHKeysImpl => {
  const [sshKeys, setSshKeys] = useState<SSHKey[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [pageSize, setPageSize] = useState<number>(10);
  const [totalCount, setTotalCount] = useState<number>(0);

  const fetchSSHKeys = async (page = 1, pageSize = 10) => {
    setIsLoading(true);
    setCurrentPage(page);
    setPageSize(pageSize);
    try {
      const response = await fetch(`/api/user/setting/ssh-key?page=${page - 1}&page_size=${pageSize}`, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });
      if (response.ok) {
        const data: Result<{ items: SSHKey[], total_count: number }> = await response.json();
        if (data.data && data.code === 200) {
          setSshKeys(data.data.items);
          setTotalCount(data.data.total_count);
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to fetch SSH keys:", error);
      toast.error("Failed to fetch SSH keys");
    } finally {
      setIsLoading(false);
    }
  };
  const addSSHKey = async (key: { title: string; key: string }) => {
    setIsLoading(true);
    try {
      const response = await fetch("/api/user/setting/ssh-key", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: key.title,
          content: key.key,
        }),
      });

      if (response.ok) {
        const data: Result<SSHKey> = await response.json();
        if (data.data && data.code === 200) {
          // 添加成功后，重新获取最新的密钥列表
          await fetchSSHKeys(currentPage, pageSize);
          toast.success("SSH key added successfully");
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to add SSH key:", error);
      toast.error("Failed to add SSH key");
    } finally {
      setIsLoading(false);
    }
  };
  const deleteSSHKey = async (name: string) => {
    setIsLoading(true);
    try {
      const response = await fetch(`/api/user/setting/ssh-key/${name}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (response.ok) {
        const data: Result<undefined> = await response.json();
        if (data.code === 200) {
          await fetchSSHKeys(currentPage, pageSize);
          toast.success("SSH key deleted successfully");
        } else {
          toast.warning(data.msg);
        }
      }
    } catch (error) {
      console.error("Failed to delete SSH key:", error);
      toast.error("Failed to delete SSH key");
    } finally {
      setIsLoading(false);
    }
  };

  return {
    sshKeys,
    isLoading,
    currentPage,
    pageSize,
    totalCount,
    fetchSSHKeys,
    addSSHKey,
    deleteSSHKey,
  };
};

export default useSSHKeys;
