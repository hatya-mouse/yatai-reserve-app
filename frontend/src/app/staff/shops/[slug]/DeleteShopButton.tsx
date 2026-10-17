"use client";

import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { deleteShop } from "@/actions/delete-shop";

export function DeleteShopButton({
    shopId,
    shopName,
}: {
    shopId: string;
    shopName: string;
}) {
    const router = useRouter();
    const [pending, startTransition] = useTransition();

    function handleDelete() {
        if (!window.confirm(`「${shopName}」を削除しますか？`)) {
            return;
        }

        startTransition(async () => {
            try {
                await deleteShop(shopId);
                router.push("/staff");
                router.refresh();
            } catch {
                window.alert("屋台の削除に失敗しました。もう一度お試しください。");
            }
        });
    }

    return (
        <Button
            type="button"
            variant="destructive"
            onClick={handleDelete}
            disabled={pending}
        >
            {pending ? "削除中..." : "屋台を削除"}
        </Button>
    );
}
