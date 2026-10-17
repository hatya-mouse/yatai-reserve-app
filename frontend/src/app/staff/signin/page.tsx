import { Suspense } from "react";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { authApi } from "@/lib/api";
import SignInForm from "./SignInForm";
import Loading from "@/app/loading";

async function AuthCheck() {
    const token = (await cookies()).get("session")?.value;

    if (token) {
        let authenticated = false;

        try {
            const response = await authApi("/staff/me");
            authenticated = response.ok;
        } catch {
            authenticated = false;
        }

        if (authenticated) {
            redirect("/staff");
        }
    }

    return <SignInForm />;
}

export default function SignInPage() {
    return (
        <Suspense fallback={<Loading />}>
            <AuthCheck />
        </Suspense>
    );
}
