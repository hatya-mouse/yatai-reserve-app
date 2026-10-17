import Loading from "@/app/loading";
import { Suspense } from "react";
import ReserveForm from "./ReserveForm";
import { getShop } from "@/lib/shop";

async function ReserveContent({
    params,
}: {
    params: Promise<{ slug: string }>;
}) {
    const slug = (await params).slug;
    let shop;

    try {
        shop = await getShop(slug);
    } catch {
        return (
            <p className="text-muted-foreground">
                予約フォームの読み込みに失敗しました。
            </p>
        );
    }

    return <ReserveForm shop={shop} />;
}

export default function ReservePage({
    params,
}: {
    params: Promise<{ slug: string }>;
}) {
    return (
        <main className="flex min-h-svh items-center justify-center bg-muted/40 px-4 py-12">
            <Suspense fallback={<Loading />}>
                <ReserveContent params={params} />
            </Suspense>
        </main>
    );
}
