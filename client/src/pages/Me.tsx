import {FormEvent, useEffect, useState} from "react"
import {useNavigate} from "react-router-dom";

export default function Me() {
    const navigate = useNavigate();
    const [password , setPassword] = useState("");
    useEffect(() => {
        try {
            fetch("/account/me", {
                method: "GET",
                credentials: "include",
            }).then(res => {
                switch (res.status) {
                    case 401:
                        navigate("/login");
                        break
                }
            })
        } catch (error) {
            console.log("Failed to get user");
        }

    }, [])
    async function handleDelete(e : FormEvent<HTMLFormElement>) {
        e.preventDefault();
        if (confirm("Are you sure you want to delete your account this is permenant")) {
            try {
                const res = await fetch("/account/delete" , {
                    method : "POST",
                    credentials : "include",
                    body : JSON.stringify({
                        password
                    })
                })
                navigate("/");
                console.log(res);
            }catch(e) {
                console.log(e)
            }
        }
    }
    return <div>
        <form onSubmit={handleDelete}>
            <input type="password" value={password} onChange={e => setPassword(e.target.value)}/>
            <button type="submit"> Delete Account </button>
        </form>
    </div>
}