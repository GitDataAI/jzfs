import {Button, Checkbox, Form, Input} from "antd";
import {useNavigate} from "react-router-dom";
import {useState} from "react";
import {toast} from "react-toastify";
import {LoginReq} from "./Api";

export const LoginPage = () => {
    const nav = useNavigate();
    const [FormData, setFormData] = useState({
        account: "",
        password: "",
    })
    const Login = () => {
        const account = FormData.account;
        const password = FormData.password;
        if (account.length === 0) {
            toast("Please input you username or email")
            return;
        }
        if (password.length < 6) {
            toast("Please input you password, password last len 6")
            return;
        }
        LoginReq(account, password)
            .then()
            .catch()
            .finally()
    }
    return(
        <div className="auth-ops">
            <h1 className="auth-title">
                Login GitDdataAI
            </h1>
            <span>No Account?&nbsp;&nbsp; <a onClick={()=>nav("/auth/register")} style={{
                color: "#0a8cff",
                cursor: "pointer"
            }}> Go to Register </a></span>
            <Form
                layout="vertical"
                className="auth-form"
            >
                <Form.Item label="Account">
                    <Input size="large" onChange={(x)=>{
                        setFormData(data=>{
                            return {
                                ...data,
                                account: x.target.value
                            }
                        })
                    }}/>
                </Form.Item>
                <Form.Item label="Password">
                    <Input.Password size="large" onChange={(e)=>{
                        setFormData(data=>{
                            return {
                                ...data,
                                password: e.target.value
                            }
                        })
                    }}/>
                </Form.Item>
                <Form.Item>
                    <div
                        style={{
                            display: 'flex',
                            justifyContent: 'space-between'
                        }}
                    >
                        <Checkbox>Remember me</Checkbox>
                        <a onClick={() => nav("/auth/forget")} style={{
                            color: '#1890ff',
                            cursor: "pointer"
                        }}>Forget Password?</a>
                    </div>

                </Form.Item>
                <Button type="primary" size="large" block onClick={Login}>Login</Button>
            </Form>
            <div>
                <span className="other-login"><p>Other Login</p></span>
            </div>
        </div>
    )
}