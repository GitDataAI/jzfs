import {Header} from "@/app/Layout.Header.tsx";
import "./module.css"
import {Outlet} from "react-router-dom";

const RootLayout = () => {
    return(
        <>
            <Header/>
            <div className="contant">
                <Outlet/>
            </div>
        </>
    )
}

export default RootLayout