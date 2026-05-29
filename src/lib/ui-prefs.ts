export type ThemeMode = "light" | "dark";
export type FontScale = "compact" | "normal";
export type UiLanguage = "es" | "en" | "fr" | "de" | "pt";

export type CardColorChoice = "none" | "blue" | "green" | "red" | "yellow" | "gray" | "orange";

export type CardColorPrefs = {
  activeDone: CardColorChoice;
  activePending: CardColorChoice;
  proposalFeature: CardColorChoice;
  proposalBug: CardColorChoice;
  archived: CardColorChoice;
};

export const DEFAULT_CARD_COLORS: CardColorPrefs = {
  activeDone: "none",
  activePending: "none",
  proposalFeature: "none",
  proposalBug: "none",
  archived: "none",
};
