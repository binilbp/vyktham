type Theme = "white" | "dark" | "catppuchin";

const themeColorMap = {
    white: {
        background: "bg-white-100", 
        muted: "text-slate-500"
    },
    dark: {
        background: "bg-slate-800",
        muted: "text-slate-400"
    },
    catppuchin: {
        background: "bg-rose-900", 
        muted: "text-rose-400"
    }
};

class AppSettings {
    theme: Theme = $state("white");

    get colors() {
        return themeColorMap[this.theme]
    }
}

export const appSettings = new AppSettings();
