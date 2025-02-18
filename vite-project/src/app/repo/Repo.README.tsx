import Markdown from "react-markdown";
import {Card, CardBody} from "@heroui/react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import rehypeRaw from 'rehype-raw';
import "./markdown.css"

interface RepoREADMEProps {
    file: Uint8Array
}

export const RepoREADME = (props: RepoREADMEProps) => {
    const str = props.file.toString();
    const markdown = str.replace(/\\n/g, "\n");
    return (
        <Card>
            <CardBody>
                <Markdown
                    className="markdown-content"
                    components={{
                        code: function ({children, className}) {
                            console.log(className)
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
                    }}
                    rehypePlugins={[rehypeRaw]}
                >{markdown}</Markdown>
            </CardBody>
        </Card>
    )
}