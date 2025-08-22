import { useState, useEffect } from "react";
import { toast } from "sonner";
import type {Result} from "@/lib/result.tsx";

export interface Profile {
  display_name?: string;
  bio?: string;
  location?: string;
  company?: string;
  timezone?: string;
  language?: string;
  theme?: string;
  website_url?: string;
  avatar?: string;
}

interface UseProfileImpl {
  profile: Profile;
  isLoading: boolean;
  fetchProfile: () => Promise<void>;
  updateProfile: (profile: Partial<Profile>) => Promise<void>;
}

const useProfile = (): UseProfileImpl => {
  const [profile, setProfile] = useState<Profile>({
    display_name: "",
    bio: "",
    location: "",
    company: "",
    timezone: "",
    language: "",
    theme: "",
    website_url: "",
    avatar: "",
  });
  const [isLoading, setIsLoading] = useState<boolean>(false);
  useEffect(() => {
    const storedState = localStorage.getItem("profile");
    if (storedState) {
      try {
        const parsedState = JSON.parse(storedState);
        setProfile(parsedState.profile || profile);
      } catch (error) {
        console.error("Failed to parse stored profile:", error);
      }
    }
  }, []);
  useEffect(() => {
    const stateToStore = {
      profile,
    };
    localStorage.setItem("profile", JSON.stringify(stateToStore));
  }, [profile]);

  // 获取用户profile数据
  const fetchProfile = async () => {
    setIsLoading(true);
    try {
      const response = await fetch("/api/user/setting/basic", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (response.ok) {
        const userSettingData:Result<Profile> = await response.json();
        if (userSettingData.code !== 200) {
          toast.error(userSettingData.msg);
          return;
        } else if (userSettingData.data) {
          const data = userSettingData.data;
          const profileData: Profile = {
            avatar: profile.avatar,
            display_name: data.display_name || profile.display_name,
            bio: data.bio || profile.bio,
            location: data.location || profile.location,
            company: data.company || profile.company,
            language: data.language || profile.language,
            theme: data.theme || profile.theme,
            website_url: data.website_url,
          };

          setProfile(profileData);
        }

      } else {
        toast.error("Failed to fetch profile");
      }
    } catch (error) {
      console.error("Failed to fetch profile:", error);
      toast.error("Failed to fetch profile");
    } finally {
      setIsLoading(false);
    }
  };

  const updateProfile = async (updatedProfile: Partial<Profile>) => {
    setIsLoading(true);
    try {
      const requestData = {
        display_name: updatedProfile.display_name,
        bio: updatedProfile.bio || updatedProfile.bio,
        location: updatedProfile.location,
        company: updatedProfile.company,
        language: updatedProfile.language,
        theme: updatedProfile.theme,
        website_url: updatedProfile.website_url,
      };
      
      const response = await fetch("/api/user/setting/basic", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(requestData),
      });

      if (response.ok) {
        // 更新成功后重新获取最新数据
        await fetchProfile();
        toast.success("Profile updated successfully");
      } else {
        toast.error("Failed to update profile");
      }
    } catch (error) {
      console.error("Failed to update profile:", error);
      toast.error("Failed to update profile");
    } finally {
      setIsLoading(false);
    }
  };

  return {
    profile,
    isLoading,
    fetchProfile,
    updateProfile,
  };
};

export default useProfile;