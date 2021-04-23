import { ReactNode } from "react";
import HelpDisplay from "./HelpDisplay";

export type CommandHandler = ReactNode | ((args: string) => void);

export interface CommandMap {
    [command: string]: CommandHandler;
}

export interface CommandOptions {
    name: string;
    description: ReactNode;
    handle: (args: string) => (ReactNode | Promise<ReactNode> | void | Promise<void>)
}

export class CommandMapBuilder {

    private optionsList: CommandOptions[] = [];

    constructor() {
        const help = {
            name: "help",
            description: "Display this list"
        }

        const helpOptions: CommandOptions = {
            ...help,
            handle: () => (
                <HelpDisplay
                    commands={this.optionsList}
                />
            )
        }

        this.optionsList.push(helpOptions);
    }

    add(name: CommandOptions["name"], description: CommandOptions["description"], handle: CommandOptions["handle"]): CommandMapBuilder {
        this.optionsList.push({name, description, handle});
        return this;
    }

    build(): CommandMap {
        let map: CommandMap = {};

        this.optionsList.forEach(options => {
            map[options.name] = options.handle
        })

        return map;
    }

}

export const download = (url: string, name: string): CommandOptions["handle"] => () => {
    fetch(url)
    .then(res => res.blob())
    .then(blob => {
        console.log(blob);

        const link = document.createElement("a");
        link.download = name;
        link.href = URL.createObjectURL(blob);
        link.click();
    })
}

export const openWindows = (amount: number, url: string): CommandOptions["handle"] => () => openWindow(amount, url)

const openWindow = (amount: number, url: string) => {
    const w = window.open(`${url}+${amount}`);
    console.log("window", w);
    
    if(amount > 1) {
        w?.addEventListener("load", () => openWindow(amount - 1, url+"+"+amount))
    }
}