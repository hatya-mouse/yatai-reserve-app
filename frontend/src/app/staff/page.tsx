"use server";

import Link from "next/link";
import { redirect } from "next/navigation";
import { serverApi } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cookies } from "next/headers";

type Shop = {
    id: string;
    name: string;
    notify_previous: boolean;
};

export async function signOut() {
    try {
        await serverApi("/staff/signout", {
            method: "POST",
        });
    } finally {
        (await cookies()).delete("session_token");
    }

    redirect("/");
}

export default async function StaffPage() {
    const response = await serverApi("/staff/shops");

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }

    if (!response.ok) {
        throw new Error("屋台一覧の取得に失敗しました");
    }

    const shops: Shop[] = await response.json();

    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">屋台管理</h1>

                <form action={signOut} method="post">
                    <Button type="submit" variant="outline">
                        ログアウト
                    </Button>
                </form>
            </header>

            <section className="space-y-4">
                <div className="flex items-center justify-between gap-4">
                    <h2 className="text-lg font-semibold">屋台一覧</h2>
                    <Button>
                        <Link href="/staff/shops/new">＋ 屋台を作成</Link>
                    </Button>
                </div>

                {shops.length === 0 ? (
                    <Card>
                        <CardContent className="py-10 text-center text-muted-foreground">
                            まだ屋台がありません。
                        </CardContent>
                    </Card>
                ) : (
                    <div className="grid gap-4 sm:grid-cols-2">
                        {shops.map((shop) => (
                            <Card key={shop.id}>
                                <CardHeader>
                                    <CardTitle>{shop.name}</CardTitle>
                                </CardHeader>
                                <CardContent className="flex items-center justify-between gap-3">
                                    <span className="text-sm text-muted-foreground">
                                        {shop.notify_previous
                                            ? "前の予約者に通知する"
                                            : "前の予約者に通知しない"}
                                    </span>

                                    <Button variant="outline">
                                        <Link href={`/staff/shops/${shop.id}`}>
                                            管理
                                        </Link>
                                    </Button>
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                )}
            </section>
        </main>
    );
}
