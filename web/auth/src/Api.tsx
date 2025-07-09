import {toast} from "react-toastify";

export async function LoginReq(username: string, password: string) {
    let payload = {
        username: username,
        password: password
    }
    const res = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
    const json = await res.json();
    const code = json.code;
    if (code === 200) {
        toast("Login Success");
    } else {
        toast(`Login Failed: \`${json.msg}\``);
    }
}

export async function RegisterReq(username: string, password: string, email: string) {
    let payload = {
        username: username,
        password: password,
        email: email
    }
    const res = await fetch(`/api/auth/register`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
    const json = await res.json();
    const code = json.code;
    if (code === 200) {
        toast("Register Success");
        return true;
    } else {
        toast(`Register Failed: \`${json.msg}\``);
        return false;
    }
}

export async function ForgetReq(account: string) {

}

export async function SendEmailCodeReq(email: string) {
    const res = await fetch(`/api/auth/email/captcha`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            email: email
        })
    });
    const json = await res.json();
    const code = json.code;
    if (code === 200) {
        toast("Send Email Code Success");
        return true
    } else {
        toast(`Code send Failed: \`${json.msg}\``)
        return false
    }
}

export async function VerifyEmailCodeReq(email: string, captcha: string) {
    const res = await fetch(`/api/auth/email/captcha/verify`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            email: email,
            captcha: captcha
        })
    });
    const json = await res.json();
    const code = json.code;
    if (code !== 200) {
        toast(`Verify Email Code Failed: \`${json.msg}\``)
        return false;
    } else {
        return true;
    }
}

export async function CheckUserReq(username?: string, email?: string) {
    const res = await fetch(`/api/users/check`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username: username,
            email: email
        })
    });
    const json = await res.json();
    return json.code as number;
}