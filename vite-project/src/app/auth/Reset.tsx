import React, {Component} from 'react';


interface ResetState {
    username: string;
    token: string;
    passwd: string;
    password: string;
    token_ok: boolean;
    email: string;
}

interface ResetProps {
    setPosition: (position: ("login" | "apply" | "reset")) => void
}

class Reset extends Component<ResetProps, ResetState> {
    override state: ResetState = {
        username: '',
        token: '',
        passwd: '',
        password: '',
        token_ok: false,
        email: '',
    };

    override componentDidMount(): void {
        const params = new URLSearchParams(location.search);
        this.setState({
            username: params.get('username') || '',
            token: params.get('token') || '',
        });
    }

    private setPasswd = (e: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({passwd: e.target.value});
    };

    private setPassword = (e: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({password: e.target.value});
    };

    private setEmail = (e: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({email: e.target.value});
    };

    override render(): React.ReactNode {
        return (
            <div>
                <br/>
                <br/>
                <h1 className="reset-title">重新设置您的密码</h1>
                <form onSubmit={(e) => e.preventDefault()}>
                    <br/>
                    <br/>
                    {this.state.token_ok ? (
                        <>
                            <div className="reset-field">
                                <label className="reset-label">密码</label>
                                <input
                                    className="reset-input"
                                    type="password"
                                    placeholder="请输入您的密码"
                                    value={this.state.passwd}
                                    onChange={this.setPasswd}
                                />
                            </div>
                            <div className="reset-field">
                                <label className="reset-label">确认密码</label>
                                <input
                                    className="reset-input"
                                    type="password"
                                    placeholder="请输入再次确认您的密码"
                                    value={this.state.password}
                                    onChange={this.setPassword}
                                />
                            </div>
                            <div className="reset-field">
                                <button
                                    className="reset-login-button"
                                    type="button"
                                    onClick={() => {
                                    }}
                                >
                                    确认
                                </button>
                            </div>
                        </>
                    ) : (
                        <>
                            <div className="reset-field">
                                <label className="reset-label">邮箱地址</label>
                                <input
                                    className="reset-input"
                                    type="email"
                                    placeholder="请输入您的邮箱地址"
                                    value={this.state.email}
                                    onChange={this.setEmail}
                                />
                            </div>
                            <div className="reset-field">
                                <button
                                    className="reset-login-button"
                                    type="button"
                                    onClick={() => {
                                    }}
                                >
                                    发送邮件
                                </button>
                            </div>
                        </>
                    )}
                    <br/>
                    <div className="reset-field">
                        <div onClick={()=>{
                            this.props.setPosition('login')
                        }} style={{textDecoration: 'none'}}>
                            <button className="reset-apply-button" type="button">
                                返回登录
                            </button>
                        </div>
                    </div>
                </form>
                <br/>
                <br/>
                <div className="reset-origin-readme">
                    GitData.AI
                    是一个用于数据产品(例如AI模型)的开发、管理、交易的一站式协作平台，帮助您高效地开发和探索数据产品。
                </div>
                <div className="reset-origin-readme">
                    © 2023 GitData.AI &nbsp;
                    <b className="reset-this-link">隐私政策</b>&nbsp;
                    <b className="reset-this-link">服务条款</b>&nbsp;
                </div>
                <br/>
                <br/>
            </div>
        );
    }
}

export default Reset;
