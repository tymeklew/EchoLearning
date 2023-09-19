import React, { useState } from "react";
import { useParams, useSearchParams ,  useNavigate } from "react-router-dom"

export default function Reset() {
    const navigate = useNavigate();
    const [password, setPassword] = useState("");
    const [searchParams] = useSearchParams();
    const secret = searchParams.get("secret");
    const { reset_id } = useParams();

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault()
        const res = await fetch(`/reset/password/${reset_id}?secret=${secret}`, {
            method: "POST",
            body: JSON.stringify({ password })
        })
        console.log(res);
        console.log(res.status);
        switch (res.status) {
            case 200:
                navigate("/login");
                break;
            default :
                break;
        }
    }
    return <div>
        <form onSubmit={handleSubmit}>
            <input type="password" className="form-element" value={password} onChange={e => setPassword(e.target.value)} />
            <button type="submit"> Reset </button>
        </form>
    </div>
}