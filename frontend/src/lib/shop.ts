import { redirect } from "next/navigation";
import { Members, Shop, staffApi } from "./api";

export async function getShop(shop_id: string) {
    const response = await staffApi(`/staff/shops/${shop_id}`, {
        method: "GET",
    });

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("屋台情報の取得に失敗しました");
    }

    return (await response.json()) as Shop;
}

export async function getMembers(shop_id: string) {
    const response = await staffApi(`/staff/shops/${shop_id}/members`, {
        method: "GET",
    });

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("屋台のメンバーの取得に失敗しました");
    }

    return (await response.json()) as Members;
}
