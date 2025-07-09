import {Button, Checkbox, Form, Input} from "antd";
import {useNavigate} from "react-router-dom";

export const ForgetPage = () => {
    const nav = useNavigate();
    return(
        <div className="auth-ops">
            <h1 className="auth-title">
                Forget Password
            </h1>
            <span>Remember? &nbsp;&nbsp; <a onClick={()=>nav("/auth/login")} style={{
                color: "#0a8cff",
                cursor: "pointer"
            }}> Back to login </a></span>
            <Form
                layout="vertical"
                className="auth-form"
            >
                <Form.Item label="Email or Username">
                    <Input size="large"/>
                </Form.Item>
                <Button type="primary" size="large" block>Send Email</Button>
            </Form>
        </div>
    )
}