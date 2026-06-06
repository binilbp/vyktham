const themeColorMap = {
    white: {
        background: "bg-white-100", 
        heading: "text-slate-500",
        border: "border-slate-300",
        hover: "hover:text-slate-600",
        muted: "text-slate-400"
    },
    dark: {
        background: "bg-slate-800",
        heading: "text-slate-500",
        hover: "text-slate-500",
        muted: "text-slate-400"
    },
};

class AppSettings {
    theme = $state("white");

    get colors() {
        return themeColorMap[this.theme];
    }
    
    font = $state("jetbrains");
}

export const appSettings = new AppSettings();
