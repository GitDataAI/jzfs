import {Outlet} from "react-router-dom";

export const AuthLayout = () => {
    return (
        <div className="auth">
            <div className="auth-window">
                <Outlet/>
            </div>
        </div>
    )
}