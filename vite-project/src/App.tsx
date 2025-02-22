import {Route, Routes} from "react-router-dom";
import RootLayout from "@/app/Layout.tsx";
import LayoutUser from "@/app/user/layout.tsx";
import RepoLayout from "@/app/repo/layout.tsx";
import ExploreLayout from "@/app/explore/layout.tsx";
import MarketLayout from "@/app/market/layout.tsx";
import CommunityLayout from "@/app/community/layout.tsx";
import useUser from "@/state/useUser.tsx";
import {useEffect} from "react";
import {UserApi} from "@/api/UserApi.tsx";
import {AppWrite} from "@/api/Http.tsx";
import {UserModel} from "@/types.ts";
import {toast} from "@pheralb/toast";

function App() {
    const api = new UserApi();
    const user = useUser();
    useEffect(() => {
        const handle = async () => {
            const now = await api.GetNow();
            const app:AppWrite<UserModel> = JSON.parse(now.data);
            if (app.code === 401) {
                if (user.dash !== undefined || user.user !== undefined) {
                    user.logout();
                    toast.info({
                        text: "Login expired, please log in again",
                    })
                }
            }
        };
        handle().then().catch()
    }, []);
    return (
        <Routes>
            <Route path={"/"} element={<RootLayout/>}>
                <Route path={"explore"} element={<ExploreLayout/>}/>
                {
                    user.dash && (
                        <Route path={""} element={<ExploreLayout/>}/>
                    )
                }
                <Route path={"market"} element={<MarketLayout/>}/>
                <Route path={"community"} element={<CommunityLayout/>}/>
                <Route path={":username"} element={<LayoutUser/>}>
                </Route>
                <Route path={":owner/:repo"} element={<RepoLayout/>}>
                </Route>
            </Route>
        </Routes>
    )
}

export default App
