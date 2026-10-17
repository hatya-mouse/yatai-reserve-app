"use server";

import { authApi } from "@/lib/api";

export async function createNewShop(
    name: string,
    notifyPrevious: boolean,
): Promise<boolean> {
    return (await authApi("/staff/shops", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, notify_previous: notifyPrevious }),
    })).ok;
}
