import {Button, Checkbox, Form, Input} from "antd";
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {CheckUserReq, RegisterReq, SendEmailCodeReq, VerifyEmailCodeReq} from "./Api";

export const RegisterPage = () => {
    const nav = useNavigate();
    const [SendSec, setSendSec] = useState<number>(0);
    const [FormData, setFormData] = useState({
        username: "",
        password: "",
        email: "",
        captcha: "",
    });
    const [UserNameError, setUserNameError] = useState("");
    const [EmailError, setEmailError] = useState("");
    useEffect(() => {
        if (SendSec <= 0) return;

        const timer = setTimeout(() => {
            setSendSec(prev => prev - 1);
        }, 1000);

        return () => clearTimeout(timer);
    }, [SendSec]);

    const sendEmailCode = () => {
        if (SendSec <= 0) {
            setSendSec(5);
        }
    };

    const handleCheckUser = async () => {
        const res = await CheckUserReq(FormData.username, FormData.email);
        if (res === 1) {
            setUserNameError("Username already exists")
            return false;
        }
        if (res === 2) {
            setEmailError("Email already exists")
            return false;
        }
        if (res === 3) {
            setEmailError("Email already exists")
            setUserNameError("Username already exists")
            return false;
        }
        setEmailError("")
        setUserNameError("")
        return true;
    }

    const SendEmailCode = async () => {
        if (await handleCheckUser()) {
            await SendEmailCodeReq(FormData.email)
            sendEmailCode();
        }
    }

    const [DisableTable, setDisableTable] = useState(false);
    const CheckCaptcha = async () => {
        setDisableTable(true);
        if (!await VerifyEmailCodeReq(FormData.email, FormData.captcha)) {
            setDisableTable(false);
        }
    }
    useEffect(() => {
        if (FormData.captcha.length === 6) {
            CheckCaptcha();
        }
    }, [FormData.captcha]);

    const handleSubmit = async () => {
        if (await RegisterReq(FormData.username, FormData.password, FormData.email)) {
            nav('/');
        }
    }

    return(
        <div className="auth-ops">
            <h1 className="auth-title">
                Join GitDdataAI
            </h1>
            <span>Already have an account? &nbsp;&nbsp; <a onClick={()=>nav("/auth/login")} style={{
                color: "#0a8cff",
                cursor: "pointer"
            }}> Go to Login</a></span>
            <Form
                layout="vertical"
                className="auth-form"
            >
                <Form.Item label="UserName" extra={UserNameError}>
                    <Input
                        disabled={SendSec != 0 || DisableTable}
                        size="large"
                        onChange={(e) => {
                            setFormData(data=>{
                                return {
                                    ...data,
                                    username: e.target.value
                                }
                            })
                        }}
                    />
                </Form.Item>
                <Form.Item label="Password">
                    <Input.Password
                        size="large"
                        disabled={SendSec != 0 || DisableTable}
                        onChange={(e)=>{
                            setFormData(data=>{
                                return {
                                    ...data,
                                    email: e.target.value
                                }
                            })
                        }}
                    />
                </Form.Item>
                <Form.Item label="Email" extra={EmailError}>
                    <Input
                        size="large"
                        disabled={SendSec != 0 || DisableTable}
                        onChange={(e)=>{
                            setFormData(data=>{
                                return {
                                    ...data,
                                    email: e.target.value
                                }
                            })
                        }}
                    />
                </Form.Item>
                <Form.Item label="Captcha">
                    <div style={{
                        display: 'flex',
                        alignItems: 'center',
                    }}>
                        <Input
                            size="large"
                            onChange={(e)=>{
                                setFormData(data=>{
                                    return {
                                        ...data,
                                        captcha: e.target.value
                                    }
                                })
                            }}
                        />
                        <Button size="large" onClick={() => {
                            SendEmailCode()
                                .catch()
                                .then()
                                .finally()
                        }}>
                            {SendSec === 0 ? 'Send' : `${SendSec}s`}
                        </Button>
                    </div>
                </Form.Item>
                <Button type="primary" size="large" block onClick={handleSubmit}>Login</Button>
            </Form>
        </div>
    )
}