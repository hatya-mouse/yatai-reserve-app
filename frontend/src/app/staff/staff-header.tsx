import { Button } from "@/components/ui/button";
import { Me } from "@/lib/api";
import { signOut } from "@/actions/signout";
import Link from "next/link";

export default function StaffHeader({
    me,
    title = "",
}: {
    me: Me;
    title?: string;
}) {
    return (
        <header className="flex items-center justify-between gap-4">
            <h1 className="text-2xl font-bold">{title}</h1>

            <div className="flex flex-row items-center gap-4">
                <Link href="/staff/me">{me.name}</Link>

                <form action={signOut}>
                    <Button type="submit" variant="outline">
                        サインアウト
                    </Button>
                </form>
            </div>
        </header>
    );
}
