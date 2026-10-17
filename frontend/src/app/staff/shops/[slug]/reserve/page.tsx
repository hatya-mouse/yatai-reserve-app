import { redirect } from "next/navigation";
import StaffHeader from "../../../staff-header";
import { getMe } from "@/lib/me";
import { getShop } from "@/lib/shop";
import { getReservations } from "@/lib/reservation";
import { ReservationList } from "./ReservationList";

export default async function ReservePage({
    params,
}: {
    params: Promise<{ slug: string }>;
}) {
    const { slug } = await params;

    const me = await getMe();
    const shop = await getShop(slug);
    const result = await getReservations(slug);

    if (!me || !shop || !result) {
        redirect("/");
    }

    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-6">
            <StaffHeader me={me} />

            <div>
                <h1 className="text-2xl font-bold">予約管理</h1>
                <p className="text-sm text-muted-foreground">{shop.name}</p>
            </div>

            <ReservationList
                shopId={shop.id}
                reservations={result.reservations}
            />
        </main>
    );
}
