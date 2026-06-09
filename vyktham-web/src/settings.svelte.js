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


import { z } from "zod";

export const SettingsSchema = z.object({
    theme: z.enum(["white", "dark"]).default("white"),
    font: z.enum(["font-jetbrains", "font-nunito"]).default("font-jetbrains"),
});

class AppSettings {
    #state = $state(SettingsSchema.parse({}));

    get theme() {
        return this.#state.theme;
    }

    set theme(incomingValue) {
        const result = SettingsSchema.shape.theme.safeParse(incomingValue);
        if (result.success) {
            this.#state.theme = result.data;
            // localStorage.setItem("app_settings", JSON.stringify(this.#state));
        } else {
            console.warn("Invalid theme:", incomingValue);
        }
    }

    get colors() {
        return themeColorMap[this.theme];
    }
    get font() {
        return this.#state.font;
    }

    set font(incomingValue) {
        const result = SettingsSchema.shape.font.safeParse(incomingValue);
        if (result.success) {
            this.#state.font = result.data;
        } else {
            console.warn("Invalid font:", incomingValue);
        }
    }

}

export const appSettings = new AppSettings();
