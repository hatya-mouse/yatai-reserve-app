import { Suspense } from "react";
import Loading from "./loading";
import StatusRequestForm from "./StatusRequestForm";

export default function Home() {
    return (
        <Suspense fallback={<Loading />}>
            <StatusRequestForm />
        </Suspense>
    );
}
