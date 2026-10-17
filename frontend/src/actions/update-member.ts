"use server";

import { authApi } from "@/lib/api";

export async function updateMember(
    shop_id: string,
    email: string,
    is_admin: boolean,
): Promise<boolean> {
    return (await authApi(`/staff/shops/${shop_id}/members`, {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, is_admin }),
    })).ok;
}
