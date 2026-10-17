import { fetchBackend } from "./api";

export type ReservationStatus =
    | "waiting"
    | "called"
    | "cancelled"
    | "completed";

export type StatusItem = {
    id: number;
    shop_id: string;
    shop_name: string;
    status: ReservationStatus;
    created_at: number;
    position: number | null;
};

export type StatusResponse = {
    reservations: StatusItem[];
};

export async function getReservationStatus(
    token: string,
): Promise<StatusResponse | null> {
    const response = await fetchBackend(
        `/user/status?token=${encodeURIComponent(token)}`,
        { cache: "no-store" },
    );

    if (!response.ok) {
        return null;
    }

    return (await response.json()) as StatusResponse;
}
