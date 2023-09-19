import React, { useState } from "react";
import {useNavigate} from "react-router-dom";

interface Message {
    message: string,
    error: boolean,
}

export default function Register() {
    const navigate = useNavigate();
    const [pending, setPending] = useState(false)
    const [email, setEmail] = useState("");
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const [message, setMessage] = useState<null | Message>(null)
    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        setPending(true)
        try {
            let res = await fetch("/auth/register", {
                method: "POST",
                body: JSON.stringify({
                    email, password, name
                })
            })
            console.log(res);
            switch (res.status) {
                case 201:
                    navigate("/login");
                    break
                case 409:
                    setMessage({
                        message: "User with that email already exists",
                        error: true,
                    })
                    break;
                case 404:
                    setMessage({
                        message: "Something silly is going on",
                        error: true,
                    })
                    break;
            }
        } catch (error) {
        }
        setPending(false);

    }
    return <div className="w-screen h-screen flex items-center justify-center">
        <div className="w-[30%] h-[40%] flex flex-col items-center ">
            <form onSubmit={handleSubmit} className="grid grid-cols-2 grid-rows-4 gap-2">
                <h1 className="col-span-2 text-center text-[25px]">Register</h1>
                <input type="email" value={email} onChange={e => setEmail(e.target.value)} placeholder="Email" required disabled={pending} className="form-element col-start-1 col-span-2" />
                <input type="text" value={name} onChange={e => setName(e.target.value)} placeholder="Name" required disabled={pending} className="form-element" />
                <input type="password" value={password} onChange={e => setPassword(e.target.value)} placeholder="Password" required disabled={pending} className="form-element" />
                <button type="submit" className="col-start-2  border-white border-[2px] border-solid p-4 rounded-md flex items-center justify-center" disabled={pending}>Sign Up</button>
            </form>
            <div className="flex items-center justify-around">
                <a href="/login"> Login instead</a>
            </div>
            <div className={`${message == null ? "hidden" : "block"} text-[20px] w-full ${message?.error ? "bg-red-500" : "bg-green-500"} p-4 rounded-md flex items-center justify-center mt-4`}> {message?.message != "" ? message?.message : ""}  </div>
        </div>


    </div>
}