import {Outlet} from "react-router-dom";
import {ToastContainer} from "react-toastify";

export const AuthLayout = () => {
    return (
        <div className="auth">
            <ToastContainer position="top-right" />
            <div className="auth-window">
                <Outlet/>
            </div>
        </div>
    )
}