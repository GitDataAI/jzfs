import {create} from "zustand";
import {createJSONStorage, persist} from "zustand/middleware/persist";

export interface BreadcrumbItem {
    label: string;
    href: string;
}

export interface BreadcrumbState {
    breadcrumb: BreadcrumbItem[];
    setBreadcrumb: (breadcrumb: BreadcrumbItem[]) => void;
    addBreadcrumb: (breadcrumb: BreadcrumbItem) => void;
    removeBreadcrumb: (index: number) => void;
    clearBreadcrumb: () => void;
}

const useBread = create<BreadcrumbState>()(persist((set) => ({
    breadcrumb: [],
    setBreadcrumb: (breadcrumb: BreadcrumbItem[]) => set({breadcrumb}),
    addBreadcrumb: (breadcrumb: BreadcrumbItem) => set((state) => ({breadcrumb: [...state.breadcrumb, breadcrumb]})),
    removeBreadcrumb: (index: number) => set((state) => ({breadcrumb: state.breadcrumb.filter((_, i) => i !== index)})),
    clearBreadcrumb: () => set({breadcrumb: []}),
}),{
    name: "breadcrumb",
    storage: createJSONStorage(() => localStorage),
}))

export default useBread;