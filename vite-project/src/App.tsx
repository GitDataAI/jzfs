import {Route, Routes} from "react-router-dom";
import RootLayout from "@/app/Layout.tsx";
import LayoutUser from "@/app/user/layout.tsx";
import RepoLayout from "@/app/repo/layout.tsx";
import ExploreLayout from "@/app/explore/layout.tsx";
import MarketLayout from "@/app/market/layout.tsx";
import CommunityLayout from "@/app/community/layout.tsx";

function App() {
    return (
        <Routes>
            <Route path={"/"} element={<RootLayout/>}>
                <Route path={"explore"} element={<ExploreLayout/>}/>
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
