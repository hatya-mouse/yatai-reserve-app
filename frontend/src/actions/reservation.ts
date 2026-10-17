"use server";

import { authApi, fetchBackend } from "@/lib/api";

export async function reserve(shop_id: string, email: string) {
    return await fetchBackend(`/user/shops/${shop_id}/reserve`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
    });
}

export async function request_status(email: string) {
    return (
        await fetchBackend("/user/request-status", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ email }),
        })
    ).ok;
}

export async function cancelReservation(reservation_id: string, token: string) {
    return (
        await fetchBackend(
            `/user/reservations/${reservation_id}/cancel?token=${token}`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
            },
        )
    ).ok;
}

export async function getStatus(token: string) {
    return (
        await fetchBackend(`/user/status?token=${token}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        })
    ).ok;
}

export async function updateReservation(
    shopId: string,
    reservationId: number,
    status: "waiting" | "called" | "cancelled" | "completed",
): Promise<boolean> {
    const response = await authApi(
        `/staff/shops/${shopId}/reservations/${reservationId}`,
        {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ status }),
        },
    );

    return response.ok;
}

export async function callReservation(
    shopId: string,
    reservationId: number,
): Promise<boolean> {
    const response = await authApi(
        `/staff/shops/${shopId}/reservations/${reservationId}`,
        {
            method: "POST",
        },
    );

    return response.ok;
}
