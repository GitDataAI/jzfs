import "./header.css"
import {styled} from "storybook/theming";
import {AiOutlineMenu} from "react-icons/ai";
import  {type ChangeEvent} from "react";
import {IoIosAdd, IoIosNotificationsOutline} from "react-icons/io";
import {Drawer, Dropdown} from "antd";
import * as React from "react";


export interface GlobalHeaderProps {
    theme: "LightMode" | "DarkMode" | "#e3e3e3",
    onclick_left_pop_menu?: () => void;
    search_input?: (e: ChangeEvent<HTMLInputElement>) => void;
    create_menu?: { url: string, name: string , icon?: React.ReactElement }[],
    search_click?: ()=> void,
    user_avatar?: string,
    user_pop_element?: React.ReactElement
    user_pop_width?: number,
    notify_url?: string,
}

export interface GlobalHeaderStyleProps {
    $background: string,
}

const GlobalHeaderStyle = styled.div<GlobalHeaderStyleProps>`
    background-color: ${props => props.$background};
    width: 100%;
    height: 64px;
    display: flex;
    justify-content: space-between;
    position: absolute;
    top: 0;
    left: 0;
`

const GlobalHeaderStyleLeft = styled.div`
    display: flex;
    justify-items: center;
    padding: 6px;
    align-items: center;
    gap: 1rem;
`

const GlobalHeaderStyleLeftLogo = styled.img`
`
const GlobalHeaderStyleLeftPopMenu = styled.button`
    height: 28px;
    border-radius: 5px;
    border: 1px #e1e1e1 solid;
    position: relative;
`

const GlobalHeaderStyleRight = styled.div`
    display: flex;
    justify-items: center;
    padding: 6px;
    align-items: center;
    gap: 1rem;
    margin-right: 2rem;
`

const GlobalHeaderStyleRightSearch = styled.input`
    height: 24px;
    border-radius: 5px;
    border: 1px #e1e1e1 solid;
    :focus {
        outline: 1px;
    }
`

const GlobalHeaderStyleRightNewBtu = styled.button`
    height: 28px;
    border-radius: 5px;
    border: 1px #e1e1e1 solid;
    position: relative;
`

export const GlobalHeader = (prop: GlobalHeaderProps) => {
    const [ShowAvatarDrawer, setShowAvatarDrawer] = React.useState(false)
    return(
        <GlobalHeaderStyle $background={
            prop.theme === "LightMode" ? "white" :
            prop.theme === "DarkMode" ? "#838383" :
            prop.theme
        }>
            <GlobalHeaderStyleLeft>
                <GlobalHeaderStyleLeftPopMenu onClick={prop.onclick_left_pop_menu}>
                    <AiOutlineMenu/>
                </GlobalHeaderStyleLeftPopMenu>
                <GlobalHeaderStyleLeftLogo src={"https://jzhub.io/gitdata-ai.png"} height="48px"/>
            </GlobalHeaderStyleLeft>
            <GlobalHeaderStyleRight>
                <GlobalHeaderStyleRightSearch
                    placeholder="Search..."
                    onChange={prop.search_input}
                    onKeyUp={(e)=>{
                        if(e.keyCode === 13){
                            prop.search_click?.();
                        }
                    }}
                />
                <Dropdown
                    menu={{
                        items: prop.create_menu?.map((item) => (
                            {
                                key: item.url,
                                label: item.name,
                                icon: item.icon,
                            }
                        )),
                        onClick: (e) => {
                            window.location.href = e.key;
                        }

                    }}
                >
                    <a onClick={(e) => e.preventDefault()}>
                        <GlobalHeaderStyleRightNewBtu>
                            <IoIosAdd/>
                        </GlobalHeaderStyleRightNewBtu>
                    </a>
                </Dropdown>
                <GlobalHeaderStyleRightNewBtu onClick={()=>{
                    if (prop.notify_url) {
                        window.location.href = prop.notify_url
                    }
                }}>
                    <IoIosNotificationsOutline/>
                </GlobalHeaderStyleRightNewBtu>
                <GlobalHeaderStyleLeftLogo src={prop.user_avatar} alt="logo" height={40} onClick={()=>{
                    setShowAvatarDrawer(true)
                }}/>
                <Drawer
                    onClose={()=>{
                        setShowAvatarDrawer(false)
                    }}
                    open={ShowAvatarDrawer}
                    closable={false}
                    width={prop.user_pop_width || 300}
                    mask={true}
                >
                    {prop.user_pop_element}
                </Drawer>
            </GlobalHeaderStyleRight>
        </GlobalHeaderStyle>
    )
}