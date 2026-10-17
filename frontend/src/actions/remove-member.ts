"use server";

import { authApi } from "@/lib/api";

export async function removeMember(
    shop_id: string,
    email: string,
): Promise<boolean> {
    return (await authApi(`/staff/shops/${shop_id}/members`, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
    })).ok;
}
