import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { serverApi } from "@/lib/api";
import SignInForm from "./sign-in-form";

export default async function Home() {
    const token = (await cookies()).get("session")?.value;

    if (token) {
        let authenticated = false;

        try {
            const response = await serverApi("/staff/me");
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
