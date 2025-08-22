import type {RouteObject} from "react-router-dom";
import PricingPage from "@/app/about/price.tsx";
import AboutLayout from "@/app/about/layout.tsx";
import {AboutHome} from "@/app/about/home.tsx";

export const AboutRoutes:RouteObject[] = [
    {
        path: "/about",
        element: <AboutLayout/>,
        children: [
            {
                path: "",
                element: <AboutHome/>
            },
            {
                path: "pricing",
                element: <PricingPage/>,
            }
        ]
    }
]


export const BasicAboutRoutes:RouteObject[] = [
    {
        path: "",
        element: <AboutLayout/>,
        children: [
            {
                path: "",
                element: <AboutHome/>
            },
            {
                path: "pricing",
                element: <PricingPage/>,
            }
        ]
    }
]