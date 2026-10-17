"use server";

import { authApi } from "@/lib/api";

export async function createNewShop(
    name: string,
    notifyPrevious: boolean,
): Promise<Response> {
    return await authApi("/staff/shops", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, notify_previous: notifyPrevious }),
    });
}
