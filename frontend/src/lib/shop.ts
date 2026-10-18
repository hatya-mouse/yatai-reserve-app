import { redirect } from "next/navigation";
import { Members, Shop, authApi, fetchBackend } from "./api";

export async function getShop(shopId: string) {
    const response = await fetchBackend(`/user/shops/${shopId}`, {
        method: "GET",
    });

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        console.log("Error on /user/shops/", response);
        throw new Error("屋台情報の取得に失敗しました");
    }

    return (await response.json()) as Shop;
}

export async function getMembers(shopId: string) {
    const response = await authApi(`/staff/shops/${shopId}/members`, {
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
