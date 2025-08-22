import {
    Archive,
    BookMarked,
    BookOpen,
    Bot,
    CircleGauge, Code,
    Database,
    EqualApproximately, GitBranch, GitCommit, GitPullRequest, LayoutDashboard, LockKeyhole, Megaphone, MessageSquare,
    MessageSquareMore, Package,
    Receipt,
    Send, Settings, SquareUserRound, Star, Tag, UserPlus
} from "lucide-react";
import type {NavItem} from "@/hooks/use-nav-data.tsx";

export const DefaultNavMain:NavItem = {
    title: "Home",
    items: [
        {
            title: "Dashboard",
            url: "/dashboard",
            icon: CircleGauge,
            isActive: true,
        },
        {
            title: "Repositories",
            url: "/repositories",
            icon: BookMarked,
        },
        {
            title: "AI Models",
            url: "/ai",
            icon: Bot
        },
        {
            title: "Datasets",
            url: "/dataset",
            icon: Database
        },
        {
            title: "Marketplace",
            url: "/marketplace",
            icon: Archive,
        },
    ]
}

export const DefaultNavFeedback:NavItem = {
    title: "Feedback",
    items: [
        {
            title: "Docs",
            url: "#",
            icon: BookOpen,
        },
        {
            title: "Contact",
            url: "#",
            icon: Send,
        },
        {
            title: "Feedback",
            url: "#",
            icon: MessageSquareMore,
        },
        {
            title: "About",
            url: "/about",
            icon: EqualApproximately,
        }
    ]
}


export const DefaultUserSetting:NavItem = {
    title: "User Setting",
    items: [
        {
            title: "Profile",
            url: "/setting/profile",
            icon: CircleGauge,
            isActive: true,
        },
        {
            title: "Account",
            url: "/setting/account",
            icon: SquareUserRound
        },
        {
            title: "Security",
            url: "/setting/security",
            icon: LockKeyhole
        },
        {
            title: "Notifications",
            url: "/setting/notifications",
            icon: Megaphone,
        },
        {
            title: "Preferences",
            url: "/setting/preferences",
            icon: Archive,
        },
        {
            title: "Billing",
            url: "/setting/billing",
            icon: Receipt,
        },
        {
            title: "SSH",
            url: "/setting/ssh",
            icon: Send,
        },
        {
            title: "AccessKey",
            url: "/setting/access-key",
            icon: MessageSquareMore,
        },
    ]
}

export function DefaultRepoNavItem(owner: string, repo: string, setting: boolean = false):NavItem {
    const nav = {
        title: `${owner}/${repo}`,
        items: [
            {
                title: "Files",
                url: `/${owner}/${repo}/files`,
                isActive: false,
                icon: Code,
            },
            {
                title: "Branches",
                url: `/${owner}/${repo}/branches`,
                icon: GitBranch,
            },
            {
                title: "Commits",
                url: `/${owner}/${repo}/commits`,
                icon: GitCommit,
            },
            {
                title: "Tags",
                url: `/${owner}/${repo}/tags`,
                icon: Tag,
            },
            {
                title: "Pull Requests",
                url: `/${owner}/${repo}/pulls`,
                icon: GitPullRequest,
            },
            {
                title: "Discussions",
                url: `/${owner}/${repo}/discussions`,
                icon: MessageSquare,
            }
        ]
    };
    if (setting) {
        nav.items.push({
            title: "Settings",
            url: `/${owner}/${repo}/settings`,
            icon: Settings,
        })
    }
    return nav;
}

export function DefaultUserNavItem(username: string):NavItem {
    return {
        title: username,
        items: [
            {
                title: "Overview",
                url: `/${username}/overview`,
                icon: LayoutDashboard,
                isActive: true,
            },
            {
                title: "Repositories",
                url: `/${username}/repos`,
                icon: BookMarked,
            },
            {
                title: "Star",
                url: `/${username}/stars`,
                icon: Star,
            },
            {
                title: "Products",
                url: `/${username}/products`,
                icon: Package,
            },
            {
                title: "Following",
                url: `/${username}/following`,
                icon: UserPlus,
            }

        ]
    }
}