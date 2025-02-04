import { type ClassValue, clsx } from "clsx"
import type { Setter } from "solid-js";
import { twMerge } from "tailwind-merge"
import { lastSavedStore } from "./ls_store";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function updateStillDefault<T>(
  stillDefault: Setter<Array<boolean>> | undefined,
  defaultIndex: number | undefined,
  value: T,
  defaultValue: T | undefined
) {
  if (stillDefault) {
    stillDefault((prev) => {
      const newPrev = [...prev];
      if (defaultIndex !== undefined) {
        newPrev[defaultIndex] = value === defaultValue;
      }
      return newPrev;
    });
  }
}

export function getValueFromLastSaved<ValKind>(defaultValueIden: keyof typeof lastSavedStore): ValKind | null {
  const val = lastSavedStore[
    defaultValueIden
  ];

  if (val !== null) {
    return val as ValKind;
  }

  return null;
}

export function getLocaleFontClass(locale: string): string {
  switch (locale) {
    case "ja":
      return "kaisei-decol";
    case "zh-tw":
      return "lxgw-wenkai-mono-tc";
    default:
      return "poppins";
  }
}