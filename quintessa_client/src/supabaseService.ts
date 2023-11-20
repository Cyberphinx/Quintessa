import type { SupabaseParams } from "$lib/model";
import { SupabaseClient, createClient } from "@supabase/supabase-js";

// Uses supabase's createClient function and the authentication configuration to create a new supabase client and returns it.
export function createSupabaseClient(keys: SupabaseParams) {
    let supabase: SupabaseClient = createClient(keys.url, keys.key);
    return supabase;
}
