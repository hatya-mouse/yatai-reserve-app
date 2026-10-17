import { Suspense } from "react";
import { redirect } from "next/navigation";
import { Shop } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import Link from "next/link";
import { getMe, getShops } from "@/lib/me";
import StaffHeader from "./staff-header";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

async function StaffContent() {
    const me = await getMe();
    const shops = await getShops();
    if (!me || !shops) redirect("/");

    return (
        <>
            <StaffHeader me={me} />
            <h2 className="text-xl font-bold">屋台管理</h2>
            <ShopList shops={shops.shops} />
        </>
    );
}

function ShopList({ shops }: { shops: Shop[] }) {
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

export default function StaffPage() {
    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-4">
            <Suspense fallback={<Loading />}>
                <StaffContent />
            </Suspense>
        </main>
    );
}
