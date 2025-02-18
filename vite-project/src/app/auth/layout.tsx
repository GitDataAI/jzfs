import React from 'react';
import "./layout.css"
import Login from "@/app/auth/Login.tsx";
import Apply from "@/app/auth/Apply.tsx";
import Reset from "@/app/auth/Reset.tsx";
import {ModalContent} from "@heroui/modal";


interface AuthLayoutProps {
  onClose?: () => void,
  position: "login" | "apply" | "reset",
  setPosition: (position: "login" | "apply" | "reset") => void
}

const AuthLayout: React.FC<AuthLayoutProps> = (props: AuthLayoutProps) => {

    return (
        <ModalContent>
          {
            props.position === "login" && <Login setPosition={props.setPosition}/>
          }
          {
            props.position === "apply" && <Apply setPosition={props.setPosition}/>
          }
          {
            props.position === "reset" && <Reset setPosition={props.setPosition}/>
          }
        </ModalContent>
    );
};

export default AuthLayout;
