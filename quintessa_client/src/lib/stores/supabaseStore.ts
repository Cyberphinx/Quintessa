import type { SupabaseParams } from "$lib/model";
import { createClient } from "@supabase/supabase-js";
import { derived, writable, type Writable } from "svelte/store";

export const supabaseCredentials: Writable<SupabaseParams> = writable();
export const supabaseClient = derived(supabaseCredentials, ($supabaseCredentials) => {
    let supabase = createClient($supabaseCredentials.url, $supabaseCredentials.key);
    return supabase;
})
