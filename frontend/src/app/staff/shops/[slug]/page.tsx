import { Suspense } from "react";
import { redirect } from "next/navigation";
import StaffHeader from "../../staff-header";
import { useRouter } from "next/router";
import { getMembers, getShop } from "@/lib/shop";
import { getMe } from "@/lib/me";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

async function ShopContent({ slug }: { slug: string }) {
    const me = await getMe();
    const shop = await getShop(slug);
    const members = await getMembers(slug);

    if (!me || !shop || !members) redirect("/");

    return (
        <div className="space-y-6">
            <StaffHeader me={me} title={shop.name} />

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
                </div>
            </section>

            {/* Member List */}
            <section className="space-y-3">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-semibold">メンバー</h2>
                    <span className="text-sm text-muted-foreground">
                        {members.members.length}人
                    </span>
                </div>

                <div className="rounded-lg border divide-y">
                    {members.members.map((member) => (
                        <div
                            key={member.email}
                            className="flex items-center justify-between gap-3 p-4"
                        >
                            <div className="min-w-0 space-y-1">
                                <p className="font-medium truncate">
                                    {member.name || member.email}
                                </p>
                                <p className="text-sm text-muted-foreground break-all">
                                    {member.email}
                                </p>
                            </div>

                            {member.is_admin && (
                                <span className="shrink-0 rounded-full bg-primary/10 px-3 py-1 text-xs font-medium text-primary">
                                    管理者
                                </span>
                            )}
                        </div>
                    ))}

                    {members.members.length === 0 && (
                        <p className="p-6 text-center text-sm text-muted-foreground">
                            メンバーはいません。
                        </p>
                    )}
                </div>
            </section>
        </div>
    );
}

export default function ShopPage() {
    const router = useRouter();
    const slug = router.query.slug;

    if (typeof slug === "string") {
        return (
            <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
                <Suspense fallback={<Loading />}>
                    <ShopContent slug={slug} />
                </Suspense>
            </main>
        );
    } else {
        return (
            <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
                slug was not found
            </main>
        );
    }
}
