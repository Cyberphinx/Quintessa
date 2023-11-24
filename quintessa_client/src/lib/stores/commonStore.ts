import type { Project, Resume, SignedMedia } from "$lib/model";
import { writable, type Writable } from "svelte/store";

export const theme: Writable<string> = writable('dark');
export const currentAnimation: Writable<number> = writable(0);
export const showFilters = writable(false);

export const isAuthenticated = writable(false);

export const loading = writable(false);
export const toaster = writable("Form submitted successfully!");

export const cv: Writable<HTMLDivElement | undefined> = writable(undefined);
export const portfolio: Writable<HTMLDivElement | undefined> = writable(undefined);
export const you: Writable<HTMLDivElement | undefined> = writable(undefined);

export const resume: Writable<Resume | undefined> = writable(undefined);
export const cvMedia: Writable<SignedMedia[] | []> = writable([]);
export const projects: Writable<Project[] | []> = writable([]);
export const projectsMedia: Writable<SignedMedia[] | []> = writable([]);

export const showCv = writable(true);


