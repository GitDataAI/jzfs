import {Route, Routes} from "react-router-dom";
import RootLayout from "@/app/Layout.tsx";
import LayoutUser from "@/app/user/layout.tsx";
import RepoLayout from "@/app/repo/layout.tsx";

function App() {
    return (
        <Routes>
            <Route path={"/"} element={<RootLayout/>}>
                <Route path={":username"} element={<LayoutUser/>}>
                </Route>
                <Route path={":owner/:repo"} element={<RepoLayout/>}>
                </Route>

            </Route>
        </Routes>
    )
}

export default App
