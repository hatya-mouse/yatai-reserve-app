import { Reservation } from "@/lib/reservation";

export const statusLabels: Record<Reservation["status"], string> = {
    waiting: "順番待ち",
    called: "呼出済み",
    cancelled: "キャンセル済み",
    completed: "完了",
};

const statusStyles: Record<Reservation["status"], string> = {
    waiting: "bg-yellow-100 text-yellow-800",
    called: "bg-blue-100 text-blue-800",
    cancelled: "bg-gray-100 text-gray-600",
    completed: "bg-green-100 text-green-800",
};

export function StatusLabel({ status }: { status: Reservation["status"] }) {
    return (
        <span
            className={`rounded-full px-3 py-1 text-xs font-medium ${
                statusStyles[status]
            }`}
        >
            {statusLabels[status]}
        </span>
    );
}
