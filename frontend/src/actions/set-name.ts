"use server";

import { staffApi } from "@/lib/api";

export async function setUserName(newName: string): Promise<boolean> {
    return (
        await staffApi("/set-name", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ name: newName }),
        })
    ).ok;
}
