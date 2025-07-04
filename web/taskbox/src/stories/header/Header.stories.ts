import {GlobalHeader} from "./Header.tsx";
import type {Meta, StoryObj} from "@storybook/react-vite";

const meta = {
    title: 'Global/Header',
    component: GlobalHeader,
    parameters: {
      layout: 'fullscreen',
    },
    tags: ['autodocs'],
    args: {},
    decorators: []
}satisfies Meta<typeof GlobalHeader>;

export default meta;
type Story = StoryObj<typeof meta>;


export const DarkMode: Story = {
    args: {
        theme: 'DarkMode',
        user_avatar: "https://ss2.bdstatic.com/70cFvXSh_Q1YnxGkpoWK1HF6hhy/it/u=3315366793,2321372572&fm=253&gp=0.jpg",
        create_menu: [
            {
                url: '/',
                name: "Create"
            },
            {
                url: '/123',
                name: "Create"
            },
        ]
    },
};

export const LightMode: Story = {
    args: {
        theme: 'LightMode',
        user_avatar: "https://ss2.bdstatic.com/70cFvXSh_Q1YnxGkpoWK1HF6hhy/it/u=3315366793,2321372572&fm=253&gp=0.jpg",
        create_menu: [
            {
                url: '/',
                name: "Create"
            },
            {
                url: '/123',
                name: "Create"
            },
        ]
    },
};

export const DefaultMode: Story = {
    args: {
        theme: "#e3e3e3",
        user_avatar: "https://ss2.bdstatic.com/70cFvXSh_Q1YnxGkpoWK1HF6hhy/it/u=3315366793,2321372572&fm=253&gp=0.jpg",
        create_menu: [
            {
                url: '/',
                name: "Create"
            },
            {
                url: '/123',
                name: "Create"
            },
        ]
    },
};