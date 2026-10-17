"use server";

import { fetchBackend } from "@/lib/api";

export async function requestSignin(email: string) {
    return (
        await fetchBackend("/request-signin", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ email }),
        })
    ).ok;
}
