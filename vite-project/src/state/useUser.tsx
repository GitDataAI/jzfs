import {create} from "zustand/react";
import {createJSONStorage, persist, devtools} from "zustand/middleware";
import {UserDashBored, UserModel} from "@/types.ts";
import {AuthApi} from "@/api/AuthApi.tsx";
import {toast} from "@pheralb/toast";
import {UserApi} from "@/api/UserApi.tsx";


export interface UserState {
    user?: UserModel;
    dash?: UserDashBored,
    isLogin: boolean;
    setUser: (user: UserModel) => void;
    setLogin: (isLogin: boolean) => void;
    logout: () => void;
    getUser: () => UserModel | undefined;
    getIsLogin: () => boolean;
    getDashBored: () => UserDashBored | undefined;
    setDashBored: (dash: UserDashBored) => void;
    syncData: () => void
}
const authApi = new AuthApi();
const userApi = new UserApi();
const useUser = create<UserState>()(
    devtools(
        persist(
            (set,get) => (
                {
                    user: undefined,
                    dash: undefined,
                    isLogin: false,
                    setUser: (user: UserModel) => set({user: user, isLogin: true}),
                    setLogin: (isLogin: boolean) => set({isLogin: isLogin}),
                    logout: () => {
                        authApi.LoginOut().then(() => {});
                        set({user: undefined, isLogin: false, dash: undefined})
                    },
                    getUser: () => get().user,
                    getIsLogin: () => get().isLogin,
                    getDashBored: () => get().dash,
                    setDashBored: (dash: UserDashBored) => set({dash: dash, user: dash.user}),
                    syncData: () => {
                        const user = get().user;
                        if (!user) {
                            return;
                        }
                        userApi.DashBoredData(user.username)
                            .then((res)=>{
                                const json = JSON.parse(res.data);
                                if (json.code === 200 && json.data && res.status) {
                                    const data = json.data;
                                    set({ dash: data, user: data.user })
                                }else {
                                    toast.error({
                                        text: "Sync Data",
                                        description: "Sync Failed",
                                    });
                                }
                            })
                    }
                }
            ),
            {
                name: 'user',
                storage: createJSONStorage(()=>localStorage)
            }
        )
    )
);


export default useUser;