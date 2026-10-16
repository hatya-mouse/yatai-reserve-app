import { Suspense } from "react";
import { redirect } from "next/navigation";
import { staffApi, Shops, Me } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { signOut } from "@/lib/signout";
import Link from "next/link";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

async function ShopList() {
    const response = await staffApi("/staff/shops");

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("ユーザー情報の取得に失敗しました");
    }

    const shops = ((await response.json()) as Shops).shops;

    return (
        <section className="space-y-4">
            <div className="flex items-center justify-between gap-4">
                <h2 className="text-lg font-semibold">屋台一覧</h2>
                <Link href="/staff/shops/new">
                    <Button>＋ 屋台を作成</Button>
                </Link>
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

                                <Link href={`/staff/shops/${shop.id}`}>
                                    <Button variant="outline">管理</Button>
                                </Link>
                            </CardContent>
                        </Card>
                    ))}
                </div>
            )}
        </section>
    );
}

export default async function StaffPage() {
    const response = await staffApi("/staff/me");

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("ユーザー情報の取得に失敗しました");
    }

    const me: Me = await response.json();

    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">屋台管理</h1>

                <div className="flex flex-row gap-4">
                    {me.name}
                    <form action={signOut}>
                        <Button type="submit" variant="outline">
                            サインアウト
                        </Button>
                    </form>
                </div>
            </header>

            <Suspense fallback={<Loading />}>
                <ShopList />
            </Suspense>
        </main>
    );
}
