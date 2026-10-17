"use server";

import { authApi } from "@/lib/api";

export async function deleteShop(
    shop_id: string,
): Promise<boolean> {
    return (await authApi(`/staff/shops/${shop_id}`, {
        method: "DELETE",
    })).ok;
}
