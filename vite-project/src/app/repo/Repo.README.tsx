import Markdown from "react-markdown";
import {Card, CardBody, CardHeader} from "@heroui/react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import rehypeRaw from 'rehype-raw';
import "./markdown.css"
import {ReactNode} from "react";

export interface RepoREADMEProps {
    file: Uint8Array,
    title?: ReactNode
}

export const RepoREADME = (props: RepoREADMEProps) => {
    const str = props.file.toString();
    const markdown = str.replace(/\\n/g, "\n");
    return (
        <Card>
            {
                props.title && <CardHeader>{props.title}</CardHeader>
            }
            <CardBody>
                <Markdown
                    className="markdown-content"
                    components={{
                        code: function ({children, className}) {
                            const match = /language-(\w+)/.exec(className || "");
                            if (match) {
                                return (
                                    <SyntaxHighlighter
                                        showLineNumbers={true}
                                        language={match && match[1]}
                                    >
                                        {String(children).replace(/\n$/, "")}
                                    </SyntaxHighlighter>
                                );
                            } else {
                                return (
                                    <code className={className}>
                                        {children}
                                    </code>
                                );
                            }
                        },
                        img({className, src, alt,height}) {
                            return (
                                <img
                                    className={className}
                                    src={src}
                                    alt={alt}
                                    height={height}
                                    style={{
                                        height: {height} + "px"
                                    }}
                                />
                            )
                        }
                    }}
                    rehypePlugins={[rehypeRaw]}
                >{markdown}</Markdown>
            </CardBody>
        </Card>
    )
}