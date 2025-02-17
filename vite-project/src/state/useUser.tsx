import {create} from "zustand/react";
import {createJSONStorage, persist, devtools} from "zustand/middleware";
import {UserDashBored, UserModel} from "@/types.ts";
import {AuthApi} from "@/api/AuthApi.tsx";
import {toast} from "@pheralb/toast";


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
}
const authApi = new AuthApi();
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
                        authApi.LoginOut().then(() => {
                            toast.success({text: '退出成功'});
                        });
                        set({user: undefined, isLogin: false})
                    },
                    getUser: () => get().user,
                    getIsLogin: () => get().isLogin,
                    getDashBored: () => get().dash,
                    setDashBored: (dash: UserDashBored) => set({dash: dash})
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