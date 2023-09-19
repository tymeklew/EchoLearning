import React, { useState } from "react";
import {  useNavigate , Link} from "react-router-dom";
import {MessageBox} from "../components/index.ts"
export default function Login() {
    const navigate = useNavigate();
    const [pending, setPending] = useState(false);
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [message , setMessage] = useState<null | {message : string , error : boolean}>(null);
    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        setPending(true);
        let res;
        try {
            res = await fetch("/auth/login", {
                method: "POST",
                credentials: "include",
                body: JSON.stringify({
                    email, password
                })
            })
            navigate("/me");
        } catch(e) {
            if (res) {
                switch (res.status) {
                    case 401:
                        setMessage({
                            message : "Incorrect Password",
                            error :true,
                        });
                        break;
                    default:
                        setMessage({
                            message : res.status.toString(),
                            error :true,
                        })
                }
            }

            setMessage({
                message : e.message,
                error : true,
            });
        }
        setPending(false)
    }
    return <div className="w-screen h-screen flex flex-col items-center justify-center">
        <h1> Login </h1>
        <form onSubmit={handleSubmit} className="grid grid-rows-2 grid-cols-1 w-[30%] gap-2">
            <input type="email" placeholder="Email" className="form-element" value={email} onChange={e => setEmail(e.target.value)} required disabled={pending} />
            <input type="password" placeholder="Password" className="form-element" value={password} onChange={e => setPassword(e.target.value)} required disabled={pending} />
            <button type="submit" className="" disabled={pending}>Login</button>
        </form>
        {message != null ? <MessageBox message={message.message} error={message.error}/> : ""}
        <Link to="/reset"> Reset Password </Link>
        <Link to="/register"> Make an account instead</Link>

    </div>
}