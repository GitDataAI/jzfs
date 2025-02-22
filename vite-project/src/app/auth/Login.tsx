import {useEffect, useState} from 'react';
import {toast} from '@pheralb/toast';
import {AuthApi} from "@/api/AuthApi.tsx";
import {UserApi} from "@/api/UserApi.tsx";
import useUser from "@/state/useUser.tsx";

interface LoginProps {
    setPosition: (position: ("login" | "apply" | "reset")) => void
}

const Login = (props: LoginProps) => {
    const api = new AuthApi();
    const user_api = new UserApi();
    const [state, setState] = useState({
        username: '',
        password: '',
        code: '',
    });

    const [Captcha, setCaptcha] = useState({
        image: LoadingCaptcha,
})
    ;

    const GetCaptcha = () => {
        api.Captcha().then((x) => {
            if (x.status !== 200) {
                toast.error({text: '验证码获取失败'});
                return;
            }
            if (!x.data) {
                toast.error({text: '验证码获取失败'});
                return;
            }
            setCaptcha({
                image: x.data
            });
        });
    };
    const user = useUser();
    const LoginAction = async () => {
        try {
            const res = await api.LoginPasswd(
                state.username,
                state.password,
                state.code
            );
            const data = JSON.parse(res.data);

            if (res.status !== 200) {
                toast.error({text: '登陆失败,请检查用户名或者密码是否正确'});
                return;
            }
            if (data.code !== 200) {
                if (data.msg === "captcha error"){
                    toast.error({text: '验证码错误'});
                }else {
                    toast.error({text: '登陆失败,请检查用户名或者密码是否正确'});

                }
                return;
            }
            toast.success({text: '登陆成功'});
            const model = await user_api.GetNow();
            user.setLogin(true);
            user.setUser(JSON.parse(model.data).data);
            const dashbored = await user_api.DashBoredData(state.username);
            user.setDashBored(JSON.parse(dashbored.data).data);
            window.location.reload();
            window.location.href = "/"
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (e) {
            toast.error({text: '登陆失败,请检查用户名或者密码是否正确'});
        }
    }

    useEffect(() => {
        GetCaptcha();
    }, []);

    return (
        <div>
            <br/>
            <br/>
            <h1 className="login-title">登录以继续</h1>
            <form onSubmit={(e) => e.preventDefault()}>
                <br/>
                <br/>
                <div className="login-field">
                    <label className="login-label">账号</label>
                    <input
                        className="login-input"
                        type="text"
                        placeholder="用户名或者邮箱"
                        value={state.username}
                        onChange={(e) => setState({...state, username: e.target.value})}
                    />
                </div>

                <div className="login-field">
                    <label className="login-label">密码</label>
                    <input
                        id="password"
                        className="login-input"
                        type="password"
                        placeholder="请输入您的密码"
                        value={state.password}
                        onChange={(e) => setState({...state, password: e.target.value})}
                    />
                </div>
                <div className="login-field">
                    <label className="login-label">验证码</label>
                    <div style={{display: 'flex'}}>
                        <input
                            className="login-input-half"
                            id="captcha"
                            type="text"
                            placeholder="请输入验证码"
                            value={state.code}
                            onChange={(e) => setState({...state, code: e.target.value})}
                        />
                        <img
                            className="login-captcha-half"
                            onClick={GetCaptcha}
                            src={Captcha.image}
                            height={35}
                            alt="captcha"
                        />
                    </div>
                </div>
                <div className="login-field">
                    <div
                        onClick={()=>{
                          props.setPosition("reset")
                        }}
                        style={{textDecoration: 'none', cursor: 'pointer'}}
                    >
                        <label
                            className="login-label"
                            style={{cursor: 'pointer'}}
                        >
                            忘记密码?
                        </label>
                    </div>
                </div>
                <div className="login-field">
                    <button
                        className="login-login-button"
                        type="button"
                        onClick={() => {
                            LoginAction().then();
                        }}
                    >
                        登录
                    </button>
                </div>
                <br/>
                <div className="login-field">
                    <div
                        onClick={() => {
                          props.setPosition("apply")
                        }}
                        style={{textDecoration: 'none', cursor: 'pointer'}}
                    >
                        <button className="login-apply-button" type="button">
                            注册
                        </button>
                    </div>
                </div>
            </form>
            <br/>
            <br/>
            <div className="login-origin-readme">
                GitData.AI 是一个用于数据产品(例如AI模型)的开发、管理、交易的一站式协作平台，帮助您高效地开发和探索数据产品。
            </div>
            <div className="login-origin-readme">
                © 2023 GitData.AI &nbsp;
                <b className="login-this-link">隐私政策</b>&nbsp;
                <b className="login-this-link">服务条款</b>&nbsp;
            </div>
            <br/>
            <br/>
        </div>
    );
};

export default Login;

export const LoadingCaptcha = "data:image/gif;base64,R0lGODlhgAASAIAAAP///8DAwICAgP//AMDAwICAgL//gP+AgP//////wCD/AAEMBMQCAgMGBwQICQsMDQ4PEBESExQVFhcYGRobHB0eHyAhIiMkJSYnKCkqKywtLi8wMTIzNDU2Nzg5Ojs8PT4/QEFCQ0RFRkdISUpLTE1OT1BRUlNUVVZXWFlaW1xdXl9gYWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXp7fH1+f4CBgoOEhYaHiImKi4yNjo+QkZKTlJWWl5iZmpucnZ6foKGio6SlpqeoqaqrrK2ur7CxsrO0tba3uLm6u7y9vr/AwcLDxMXGx8jJysvMzc7P0NHS09TV1tfY2drb3N3e3+Dh4uPk5ebn6Onq6+zt7u/w8fLz9PX29/j5+vv8/f7//wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACH5BAEKAAEALAAAAACQABIAAAT/MMhJq7046827/2AojmRpnmiqrmzrvnAsz3Rt33iu73zv/8CgcEgsGo/IpHLJbDqf0Kh0Sq1ar9isdsvter/gsHhMLpvP6LR6zW673/C4fE6v2+/4vH7P7/v/gIGCg4SFhoeIiYqLjI2Oj5CRkpOUlZaXmJmam5ydnp+goaKjpKWmp6ipqqusra6vsLGys7S1tre4ubq7vL2+v8DBwsPExcbHyMnKy8zNzs/Q0dLT1NXW19jZ2tvc3d7f4OHi4+Tl5ufo6err7O3u7/Dx8vP09fb3+Pn6+/z9/v8AAwocSLCgwYMIEypcyLChw4cQI0qcSLGixYsYM2rcyLGjx48gQ4ocSbKkyZMoU6pcybKly5cwY8qcSbOmzZs4c+rcybOnz59AgwodSrSo0aNIkypdyrSp06dQo0qdSrWq1atYs2rdyrWr169gw4odS7as2bNo06pdy7at27dw48qdS7eu3bt48+rdy7ev37+AAwseTLiw4cOIEytezLix48eQI0ueTLmy5cuYM2vezLmz58+gQ4seTbq06dOoU6tezbq169ewY8ueTbu27du4c+vezbu379/AgwsfTry48ePIkytfzry58+fQo0ufTr269evYs2vfzr279+/gw4sfT768+fPo06tfz769+/fw48ufT7++/fv48+vfz7+///8ABijggAQWaOCBCCao4IIMNujggxBGKOGEFFZo4YUYZqjhhhx26OGHIIYo4ogklmjiiSimqOKKLLbo4oswxijjjDTWaOONOOao44489ujjj0AGKeSQRBZp5JFIJqnkkkw26eSTUEYp5ZRUVmnllVhmqeWWXHbp5ZdghinmmGSWaeaZaKap5ppstunmm3DGKeecdNZp55145qnnnnz26eefgAYq6KCEFmrooYgmquiijDbq6KOQRirppJRWaumlmGaq6aacdurpp6CGKuqopJZq6qmopqrqqqy26uqrsMYq66y01mrrrbjmquuuvPbq66/ABivssMQWa+yxyCar7LLMNuvss9BGK+201FZr7bXYZqvtttx26+234IYr7rjklmvuueimq+667Lbr7rvwxivvvPTWa++9+Oar77789uvvvwAHLPDABBds8MEIJ6zwwgw37PDDEEcs8cQUV2zxxRhnrPHGHHfs8ccghyzyyCSXbPLJKKes8sost+zyyzDHLPPMNNds880456zzzjz37PPPQAct9NBEF2300UgnrfTSTDft9NNQRy311FRXbfXVWGet9dZcd+3112CHLfbYZJdt9tlop6322my37fbbcMct99x012333XjnrffefPft99+ABy744IQXbvjhiCeu+OKMN+7445BHLvnklFdu+eWYZ6755px37vnnoIcu+uikl2766ainrvrqrLfu+uuwxy777LTXbvvtuOeu++689+7778AHL/zwxBdv/PHIJ6/88sw37/zz0Ecv/fTUV2/99dhnr/323Hfv/ffghy/++OSXb/756Kev/vrst+/++/DHL//89Ndv//3456///vz37///AAygAAdIwAIa8IAITKACF8jABjrwgRCMoAQnSMEKWvCCGMygBjfIwQ568IMgDKEIR0jCEprwhChMoQpXyMIWuvCFMIyhDGdIwxra8IY4zKEOd8jDHvrwh0AMohDOh0jEIhrxiEhMohKXyMQmOvGJUIyiFKdIxSpa8YpYzKIWt8jFLnrxi2AMoxjHSMYymvGMaEyjGtfIxja68Y1wjKMc50jHOtrxjnjMox73yMc++vGPgAykIAdJyEIa8pCITKQiF8nIRjrykZCMpCQnSclKWvKSmMykJjfJyU568pOgDKUoR0nKUprylKhMpSpXycpWuvKVsIylLGdJy1ra8pa4zKUud8nLXvryl8AMpjD/h0nMYhrzmMhMpjKXycxmOvOZ0IymNKdJzWpa85rYzKY2t8nNbnrzm+AMpzjHSc5ymvOc6EynOtfJzna6853wjKc850nPetrznvjMpz73yc9++vOfAA2oQAdK0IIa9KAITahCF8rQhjr0oRCNqEQnStGKWvSiGM2oRjfK0Y569KMgDalIR0rSkpr0pChNqUpXytKWuvSlMI2pTGdK05ra9KY4zalOd8rTnvr0p0ANqlCHStSiGvWoSE2qUpfK1KY69alQjapUp0rVqlr1qljNqla3ytWuevWrYA2rWMdK1rKa9axoTata18rWtrr1rXCNq1znSte62vWueM2rXvfK/9e++vWvgA2sYAdL2MIa9rCITaxiF8vYxjr2sZCNrGQnS9nKWvaymM2sZjfL2c569rOgDa1oR0va0pr2tKhNrWpXy9rWuva1sI2tbGdL29ra9ra4za1ud8vb3vr2t8ANrnCHS9ziGve4yE2ucpfL3OY697nQja50p0vd6lr3utjNrna3y93ueve74A2veMdL3vKa97zoTa9618ve9rr3vfCNr3znS9/62ve++M2vfvfL3/76978ADrCAB0zgAhv4wAhOsIIXzOAGO/jBEI6whCdM4Qpb+MIYzrCGN8zhDnv4wyAOsYhHTOISm/jEKE6xilfM4ha7+MUwjrGMZ0zjGvOjbOMb4zjHOt4xj3vs4x8DOchCHjKRi2zkIyM5yUpeMpOb7OQnQznKUp4ylats5StjOcta3jKXu+zlL4M5zGIeM5nLbOYzoznNal4zm9vs5jfDOc5ynjOd62znO+M5z3res0EBAAA7"