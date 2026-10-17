import { Suspense } from "react";
import { redirect } from "next/navigation";
import StaffHeader from "../../staff-header";
import { getMembers, getShop } from "@/lib/shop";
import { getMe } from "@/lib/me";
import { ShopMembers } from "./ShopMembers";
import Loading from "@/app/loading";
import { Button } from "@/components/ui/button";
import Link from "next/link";

async function ShopContent({ params }: { params: Promise<{ slug: string }> }) {
    const slug = (await params).slug;
    const me = await getMe();
    const shop = await getShop(slug);
    const members = await getMembers(slug);

    if (!me || !shop || !members) redirect("/");

    return (
        <div className="space-y-6">
            <StaffHeader me={me} />
            <h2 className="text-xl font-bold">{shop.name}</h2>

            {/* Shop Information */}
            <section className="space-y-3">
                <h2 className="text-xl font-semibold">屋台情報</h2>

                <div className="rounded-lg border p-4 space-y-2">
                    <div className="flex items-center justify-between gap-4">
                        <span className="text-sm text-muted-foreground">
                            屋台名
                        </span>
                        <span className="font-medium">{shop.name}</span>
                    </div>

                    <div className="flex items-center justify-between gap-4">
                        <span className="text-sm text-muted-foreground">
                            屋台ID
                        </span>
                        <span className="font-mono text-sm">{shop.id}</span>
                    </div>

                    <div className="flex items-center justify-between gap-4">
                        <span className="text-sm text-muted-foreground">
                            予約用リンク
                        </span>
                        <span className="font-mono text-sm">
                            https://reserve.hatya.dev/r/{shop.id}
                        </span>
                    </div>
                </div>
            </section>

            <div className="flex flex-wrap gap-3">
                <Link href={`/staff/shops/${shop.id}/reservations`}>
                    <Button>予約一覧</Button>
                </Link>
            </div>

            {/* Member List */}
            <ShopMembers shopId={shop.id} members={members} />
        </div>
    );
}

export default function ShopPage({
    params,
}: {
    params: Promise<{ slug: string }>;
}) {
    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-6">
            <Suspense fallback={<Loading />}>
                <ShopContent params={params} />
            </Suspense>
        </main>
    );
}
