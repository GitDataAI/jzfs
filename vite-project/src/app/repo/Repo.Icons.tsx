import {cn} from "@heroui/react";
import React, { SVGProps } from "react";
import { JSX } from "react/jsx-runtime";
import {IoSettingsOutline} from "react-icons/io5";
import {GoFileDirectoryFill} from "react-icons/go";
import {SiWikipedia} from "react-icons/si";

export const BugIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M16.895,6.519l2.813-2.812l-1.414-1.414l-2.846,2.846c-0.233-0.166-0.473-0.321-0.723-0.454 c-1.723-0.91-3.726-0.911-5.45,0c-0.25,0.132-0.488,0.287-0.722,0.453L5.707,2.293L4.293,3.707l2.813,2.812 C6.53,7.242,6.08,8.079,5.756,9H2v2h2.307C4.242,11.495,4.2,11.997,4.2,12.5c0,0.507,0.042,1.013,0.107,1.511H2v2h2.753 c0.013,0.039,0.021,0.08,0.034,0.118c0.188,0.555,0.421,1.093,0.695,1.6c0.044,0.081,0.095,0.155,0.141,0.234l-2.33,2.33 l1.414,1.414l2.11-2.111c0.235,0.254,0.478,0.498,0.736,0.716c0.418,0.354,0.867,0.657,1.332,0.903 c0.479,0.253,0.982,0.449,1.496,0.58C10.911,21.931,11.455,22,12,22s1.089-0.069,1.618-0.204c0.514-0.131,1.017-0.327,1.496-0.58 c0.465-0.246,0.914-0.55,1.333-0.904c0.258-0.218,0.5-0.462,0.734-0.716l2.111,2.111l1.414-1.414l-2.33-2.33 c0.047-0.08,0.098-0.155,0.142-0.236c0.273-0.505,0.507-1.043,0.694-1.599c0.013-0.039,0.021-0.079,0.034-0.118H22v-2h-2.308 c0.065-0.499,0.107-1.004,0.107-1.511c0-0.503-0.042-1.005-0.106-1.5H22V9h-3.756C17.92,8.079,17.47,7.242,16.895,6.519z M8.681,7.748c0.445-0.558,0.96-0.993,1.528-1.294c1.141-0.603,2.442-0.602,3.581,0c0.569,0.301,1.084,0.736,1.53,1.295 c0.299,0.373,0.54,0.8,0.753,1.251H7.927C8.141,8.549,8.381,8.121,8.681,7.748z M17.8,12.5c0,0.522-0.042,1.044-0.126,1.553 c-0.079,0.49-0.199,0.973-0.355,1.436c-0.151,0.449-0.34,0.882-0.559,1.288c-0.217,0.399-0.463,0.772-0.733,1.11 c-0.267,0.333-0.56,0.636-0.869,0.898c-0.31,0.261-0.639,0.484-0.979,0.664s-0.695,0.317-1.057,0.41 c-0.04,0.01-0.082,0.014-0.122,0.023V14h-2v5.881c-0.04-0.009-0.082-0.013-0.122-0.023c-0.361-0.093-0.717-0.23-1.057-0.41 s-0.669-0.403-0.978-0.664c-0.311-0.263-0.604-0.565-0.871-0.899c-0.27-0.337-0.516-0.71-0.731-1.108 c-0.22-0.407-0.408-0.84-0.56-1.289c-0.156-0.463-0.276-0.946-0.356-1.438C6.242,13.544,6.2,13.022,6.2,12.5 c0-0.505,0.041-1.009,0.119-1.5h11.361C17.759,11.491,17.8,11.995,17.8,12.5z"
                fill="currentColor"
            />
        </svg>
    );
};

export const PullRequestIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M19.01 15.163V7.997C19.005 6.391 17.933 4 15 4V2l-4 3 4 3V6c1.829 0 2.001 1.539 2.01 2v7.163c-1.44.434-2.5 1.757-2.5 3.337 0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5c0-1.58-1.06-2.903-2.5-3.337zm-1 4.837c-.827 0-1.5-.673-1.5-1.5s.673-1.5 1.5-1.5 1.5.673 1.5 1.5-.673 1.5-1.5 1.5zM9.5 5.5C9.5 3.57 7.93 2 6 2S2.5 3.57 2.5 5.5c0 1.58 1.06 2.903 2.5 3.337v6.326c-1.44.434-2.5 1.757-2.5 3.337C2.5 20.43 4.07 22 6 22s3.5-1.57 3.5-3.5c0-1.58-1.06-2.903-2.5-3.337V8.837C8.44 8.403 9.5 7.08 9.5 5.5zm-5 0C4.5 4.673 5.173 4 6 4s1.5.673 1.5 1.5S6.827 7 6 7s-1.5-.673-1.5-1.5zm3 13c0 .827-.673 1.5-1.5 1.5s-1.5-.673-1.5-1.5S5.173 17 6 17s1.5.673 1.5 1.5z"
                fill="currentColor"
            />
        </svg>
    );
};

export const ChatIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M5 18v3.766l1.515-.909L11.277 18H16c1.103 0 2-.897 2-2V8c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v8c0 1.103.897 2 2 2h1zM4 8h12v8h-5.277L7 18.234V16H4V8z"
                fill="currentColor"
            />
            <path
                d="M20 2H8c-1.103 0-2 .897-2 2h12c1.103 0 2 .897 2 2v8c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2z"
                fill="currentColor"
            />
        </svg>
    );
};

export const PlayCircleIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"
                fill="currentColor"
            />
            <path d="m9 17 8-5-8-5z" fill="currentColor"/>
        </svg>
    );
};

export const LayoutIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M19 3H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zm0 2 .001 4H5V5h14zM5 11h8v8H5v-8zm10 8v-8h4.001l.001 8H15z"
                fill="currentColor"
            />
        </svg>
    );
};

export const SettingIcon = (props: JSX.IntrinsicAttributes & React.ClassAttributes<HTMLElement> & React.HTMLAttributes<HTMLElement>) => {
    return(
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        <IoSettingsOutline {...props} />
    )
}

export const FileIcon = (props: JSX.IntrinsicAttributes & React.ClassAttributes<HTMLElement> & React.HTMLAttributes<HTMLElement>) => {
    return(
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        <GoFileDirectoryFill {...props}/>
    )
}
export const WikiIcon = (props: JSX.IntrinsicAttributes & React.ClassAttributes<HTMLElement> & React.HTMLAttributes<HTMLElement>) => {
    return(
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        <SiWikipedia {...props} />
    )
}

export const TagIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M11.707 2.293A.997.997 0 0 0 11 2H6a.997.997 0 0 0-.707.293l-3 3A.996.996 0 0 0 2 6v5c0 .266.105.52.293.707l10 10a.997.997 0 0 0 1.414 0l8-8a.999.999 0 0 0 0-1.414l-10-10zM13 19.586l-9-9V6.414L6.414 4h4.172l9 9L13 19.586z"
                fill="currentColor"
            />
            <circle cx="8.353" cy="8.353" fill="currentColor" r="1.647"/>
        </svg>
    );
};

export const UsersIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M16.604 11.048a5.67 5.67 0 0 0 .751-3.44c-.179-1.784-1.175-3.361-2.803-4.44l-1.105 1.666c1.119.742 1.8 1.799 1.918 2.974a3.693 3.693 0 0 1-1.072 2.986l-1.192 1.192 1.618.475C18.951 13.701 19 17.957 19 18h2c0-1.789-.956-5.285-4.396-6.952z"
                fill="currentColor"
            />
            <path
                d="M9.5 12c2.206 0 4-1.794 4-4s-1.794-4-4-4-4 1.794-4 4 1.794 4 4 4zm0-6c1.103 0 2 .897 2 2s-.897 2-2 2-2-.897-2-2 .897-2 2-2zm1.5 7H8c-3.309 0-6 2.691-6 6v1h2v-1c0-2.206 1.794-4 4-4h3c2.206 0 4 1.794 4 4v1h2v-1c0-3.309-2.691-6-6-6z"
                fill="currentColor"
            />
        </svg>
    );
};

export const WatchersIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="m21.977 13.783-2-9A1.002 1.002 0 0 0 19 4h-3v2h2.198l.961 4.326A4.467 4.467 0 0 0 17.5 10c-1.953 0-3.603 1.258-4.224 3h-2.553c-.621-1.742-2.271-3-4.224-3-.587 0-1.145.121-1.659.326L5.802 6H8V4H5a1 1 0 0 0-.976.783l-2 9 .047.011A4.552 4.552 0 0 0 2 14.5C2 16.981 4.019 19 6.5 19c2.31 0 4.197-1.756 4.449-4h2.102c.252 2.244 2.139 4 4.449 4 2.481 0 4.5-2.019 4.5-4.5 0-.242-.034-.475-.071-.706l.048-.011zM6.5 17C5.122 17 4 15.878 4 14.5S5.122 12 6.5 12 9 13.122 9 14.5 7.878 17 6.5 17zm11 0c-1.379 0-2.5-1.122-2.5-2.5s1.121-2.5 2.5-2.5 2.5 1.122 2.5 2.5-1.121 2.5-2.5 2.5z"
                fill="currentColor"
            />
        </svg>
    );
};

export const BookIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg height="1em" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M6 22h15v-2H6.012C5.55 19.988 5 19.805 5 19s.55-.988 1.012-1H21V4c0-1.103-.897-2-2-2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3zM5 8V5c0-.805.55-.988 1-1h13v12H5V8z"
                fill="currentColor"
            />
            <path d="M8 6h9v2H8z" fill="currentColor" />
        </svg>
    );
};

export const ChevronRightIcon = (props: JSX.IntrinsicAttributes & SVGProps<SVGSVGElement>) => {
    return (
        <svg
            aria-hidden="true"
            fill="none"
            focusable="false"
            height="1em"
            role="presentation"
            stroke="currentColor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="1.5"
            viewBox="0 0 24 24"
            width="1em"
            {...props}
        >
            <path d="m9 18 6-6-6-6" />
        </svg>
    );
};

export const IconWrapper = ({children, className}:{children: React.ReactNode, className: string}) => (
    <div className={cn(className, "flex items-center rounded-small justify-center w-7 h-7")}>
        {children}
    </div>
);

export const ItemCounter = ({number}: {number: number}) => (
    <div className="flex items-center gap-1 text-default-400">
        <span className="text-small">{number}</span>
        <ChevronRightIcon className="text-xl" />
    </div>
);
