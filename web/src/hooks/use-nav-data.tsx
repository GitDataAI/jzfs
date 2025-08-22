import {create} from "zustand";
import {createJSONStorage, persist} from "zustand/middleware";
import type {LucideIcon} from "lucide-react";

export interface NavItem {
    items: {
        title: string
        url: string
        icon?: LucideIcon
        isActive?: boolean
        items?: {
            title: string
            url: string
        }[]
    }[],
    title?: string
}

export interface BreadcrumbState {
    navMain?: NavItem,
    navProject?: NavItem,
    navFeedback?: NavItem,
    setNavMain: (navMain: NavItem) => void,
    setNavProject: (navProject: NavItem) => void,
    setNavFeedback: (navFeedback: NavItem) => void,
    clearNavMain: () => void,
    clearNavProject: () => void,
    clearNavFeedback: () => void,
}

const useNavData = create<BreadcrumbState>()(persist((set) => ({
    navMain: undefined,
    navProject: undefined,
    navFeedback: undefined,
    setNavMain: (navMain) => set({navMain}),
    setNavProject: (navProject) => set({navProject}),
    setNavFeedback: (navFeedback) => set({navFeedback}),
    clearNavMain: () => set({navMain: undefined}),
    clearNavProject: () => set({navProject: undefined}),
    clearNavFeedback: () => set({navFeedback: undefined}),
}),{
    name: "nav-data",
    storage: createJSONStorage(() => localStorage),
}))

export default useNavData;
