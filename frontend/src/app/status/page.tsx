import { getReservationStatus } from "@/lib/status";
import { notFound } from "next/navigation";
import { StatusLabel } from "../StatusLabel";

type Props = {
    searchParams: Promise<{
        token?: string;
    }>;
};

function formatDate(timestamp: number) {
    return new Intl.DateTimeFormat("ja-JP", {
        dateStyle: "medium",
        timeStyle: "short",
        timeZone: "Asia/Tokyo",
    }).format(new Date(timestamp * 1000));
}

export default async function StatusPage({ searchParams }: Props) {
    const { token } = await searchParams;

    if (!token) {
        notFound();
    }

    const data = await getReservationStatus(token);

    if (!data) {
        return (
            <main className="mx-auto max-w-2xl px-4 py-10">
                <h1 className="text-2xl font-bold">
                    予約状況を取得できませんでした
                </h1>
                <p className="mt-3 text-muted-foreground">
                    リンクの有効期限が切れているか、予約情報が見つかりません。
                </p>
            </main>
        );
    }

    return (
        <main className="mx-auto min-h-svh w-full max-w-2xl space-y-6 px-4 py-8">
            <header className="space-y-2">
                <h1 className="text-2xl font-bold">予約状況</h1>
                <p className="text-sm text-muted-foreground">
                    各屋台の予約状況を確認できます。
                </p>
            </header>

            {data.reservations.length === 0 ? (
                <div className="rounded-lg border p-6 text-center">
                    予約はありません。
                </div>
            ) : (
                <div className="space-y-4">
                    {data.reservations.map((reservation) => (
                        <section
                            key={`${reservation.shop_id}-${reservation.id}`}
                            className="space-y-4 rounded-xl border p-5"
                        >
                            <div className="flex items-start justify-between gap-4">
                                <div className="min-w-0">
                                    <h2 className="text-lg font-semibold">
                                        {reservation.shop_name}
                                    </h2>
                                    <p className="text-sm text-muted-foreground">
                                        予約番号：{reservation.id}
                                    </p>
                                </div>

                                <StatusLabel status={reservation.status} />
                            </div>

                            {reservation.status === "waiting" &&
                                reservation.position !== null && (
                                    <div className="rounded-lg bg-muted/50 p-4">
                                        <p className="text-sm text-muted-foreground">
                                            あなたの前に待っている人数
                                        </p>
                                        <p className="mt-1 text-2xl font-bold">
                                            {reservation.position}人
                                        </p>
                                    </div>
                                )}

                            {reservation.status === "called" && (
                                <p className="font-medium">
                                    お呼び出し中です。屋台へお越しください！
                                </p>
                            )}

                            <p className="text-xs text-muted-foreground">
                                予約日時：{formatDate(reservation.created_at)}
                            </p>
                        </section>
                    ))}
                </div>
            )}
        </main>
    );
}
