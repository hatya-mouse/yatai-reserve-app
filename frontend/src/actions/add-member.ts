"use server";

import { authApi } from "@/lib/api";

export async function addMember(
    shop_id: string,
    email: string,
    isAdmin: boolean,
): Promise<boolean> {
    return (
        await authApi(`/staff/shops/${shop_id}/members`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ email, is_admin: isAdmin }),
        })
    ).ok;
}
