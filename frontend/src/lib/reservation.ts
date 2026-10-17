import "server-only";
import { authApi } from "@/lib/api";

export type Reservation = {
    id: number;
    shop_id: string;
    email: string;
    status: "waiting" | "called" | "cancelled" | "completed";
    created_at: number;
};

export type ReservationItems = {
    reservations: Reservation[];
};

export async function getReservations(
    shopId: string,
): Promise<ReservationItems | null> {
    const response = await authApi(`/staff/shops/${shopId}/reservations`);

    if (!response.ok) {
        return null;
    }

    return response.json() as Promise<ReservationItems>;
}
