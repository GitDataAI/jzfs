import {Route, Routes} from "react-router-dom";
import AuthLayout from "@/app/auth/layout.tsx";
import Apply from "@/app/auth/Apply.tsx";
import Reset from "@/app/auth/Reset.tsx";
import Login from "@/app/auth/Login.tsx";
import RootLayout from "@/app/Layout.tsx";
import LayoutUser from "@/app/user/layout.tsx";
import RepoLayout from "@/app/repo/layout.tsx";

function App() {
    return (
        <Routes>
            <Route path={"/auth"} element={<AuthLayout/>}>
                <Route path={"login"} element={<Login/>}/>
                <Route path={"reset"} element={<Reset/>}/>
                <Route path={"apply"} element={<Apply/>}/>
            </Route>
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
