import { Button } from "@/components/ui/button";
import { Me } from "@/lib/api";
import { signOut } from "@/actions/signout";
import Link from "next/link";
import { Home } from "lucide-react";

export default function StaffHeader({ me }: { me: Me }) {
    return (
        <header className="flex items-center justify-between gap-4">
            <Link href="/staff" className="text-xl font-bold">
                <Home className="visible sm:invisible" />
                <h1 className="invisible sm:visible">工嶺祭予約アプリ</h1>
            </Link>

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
