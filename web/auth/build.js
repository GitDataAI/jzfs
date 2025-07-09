import { exec } from "node:child_process";
import {copyFile} from "node:fs";
import * as fs from "node:fs";

const cmd = "pnpm run build";

exec(cmd, { cwd: "." }, (error, stdout, stderr) => {
    if (error) {
        console.error(`exec error: ${error}`);
        return;
    }
    console.log(`stdout: ${stdout}`);
})

if (!fs.existsSync("../dist")) {
    fs.mkdirSync("../dist");
}

if (!fs.existsSync("../dist/auth")) {
    fs.mkdirSync("../dist/auth");
} else {
    fs.rmdirSync("../dist/auth", { recursive: true });
    fs.mkdirSync("../dist/auth");
}

fs.cpSync("dist", "../dist/auth",{ recursive: true })