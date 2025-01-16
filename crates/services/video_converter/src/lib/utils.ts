import { type ClassValue, clsx } from "clsx"
import type { Setter } from "solid-js";
import { twMerge } from "tailwind-merge"

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