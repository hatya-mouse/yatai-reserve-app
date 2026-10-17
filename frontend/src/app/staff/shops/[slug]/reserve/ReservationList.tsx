"use client";

import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Reservation } from "@/lib/reservation";
import { callReservation, updateReservation } from "@/actions/reservation";
import { StatusLabel, statusLabels } from "@/app/StatusLabel";

export function ReservationList({
    shopId,
    reservations,
}: {
    shopId: string;
    reservations: Reservation[];
}) {
    const router = useRouter();
    const [pending, startTransition] = useTransition();
    const [message, setMessage] = useState("");

    function changeStatus(
        reservationId: number,
        status: Reservation["status"],
    ) {
        setMessage("");

        startTransition(async () => {
            try {
                const success = await updateReservation(
                    shopId,
                    reservationId,
                    status,
                );

                if (!success) {
                    setMessage("ステータスの更新に失敗しました。");
                    return;
                }

                setMessage("ステータスを更新しました。");
                router.refresh();
            } catch {
                setMessage("ステータスの更新に失敗しました。");
            }
        });
    }

    function call(reservationId: number) {
        setMessage("");

        startTransition(async () => {
            try {
                const success = await callReservation(shopId, reservationId);

                if (!success) {
                    setMessage("呼び出しに失敗しました。");
                    return;
                }

                setMessage("予約を呼び出しました。");
                router.refresh();
            } catch {
                setMessage("呼び出しに失敗しました。");
            }
        });
    }

    return (
        <section className="space-y-4">
            <div className="flex items-center justify-between gap-3">
                <h2 className="text-xl font-semibold">予約一覧</h2>
                <span className="text-sm text-muted-foreground">
                    {reservations.length}件
                </span>
            </div>

            {message && (
                <p role="status" className="text-sm text-muted-foreground">
                    {message}
                </p>
            )}

            {reservations.length === 0 ? (
                <div className="rounded-lg border p-8 text-center text-muted-foreground">
                    予約はまだありません。
                </div>
            ) : (
                <div className="space-y-3">
                    {reservations.map((reservation) => (
                        <article
                            key={reservation.id}
                            className="space-y-3 rounded-lg border p-4"
                        >
                            <div className="flex flex-wrap items-start justify-between gap-3">
                                <div className="min-w-0 space-y-1">
                                    <p className="font-semibold">
                                        予約 #{reservation.id}
                                    </p>
                                    <p className="break-all text-sm text-muted-foreground">
                                        {reservation.email}
                                    </p>
                                    <p className="text-xs text-muted-foreground">
                                        {new Date(
                                            reservation.created_at * 1000,
                                        ).toLocaleString("ja-JP", {
                                            timeZone: "Asia/Tokyo",
                                        })}
                                    </p>
                                </div>

                                <StatusLabel status={reservation.status} />
                            </div>

                            <div className="flex flex-wrap gap-2">
                                {reservation.status === "waiting" && (
                                    <Button
                                        size="sm"
                                        disabled={pending}
                                        onClick={() => call(reservation.id)}
                                    >
                                        呼び出す
                                    </Button>
                                )}

                                {(
                                    [
                                        "waiting",
                                        "called",
                                        "cancelled",
                                        "completed",
                                    ] as const
                                )
                                    .filter(
                                        (status) =>
                                            status !== reservation.status,
                                    )
                                    .map((status) => (
                                        <Button
                                            key={status}
                                            size="sm"
                                            variant="outline"
                                            disabled={pending}
                                            onClick={() =>
                                                changeStatus(
                                                    reservation.id,
                                                    status,
                                                )
                                            }
                                        >
                                            {statusLabels[status]}
                                        </Button>
                                    ))}
                            </div>
                        </article>
                    ))}
                </div>
            )}
        </section>
    );
}
