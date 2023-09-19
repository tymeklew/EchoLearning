import {useEffect, useState} from "react";
import {Link, useLocation, useNavigate} from "react-router-dom";
interface UserDetails {
    name : string
}

export default function NavBar() {
    const navigate = useNavigate();
    const location = useLocation()
    const [user , setUser] = useState<null | UserDetails>(null);
    useEffect(() => {
        try {
            fetch("/account/me" , {
                method : "GET",
                credentials : "include"
            }).then(res => {
                console.log(res);
                switch (res.status) {
                    case 200:
                        console.log("ALL GOOD");
                        res.json().then(data => {
                            setUser({ name : data.name});
                            console.log("Set user")})
                        break;
                    default :
                        console.log("REAL");
                        break;
                }
            })
        }catch {
            console.log("Failed to get user")
        }
    }, [location]);
    async function handleLogOut() {
        try {
            const res = await fetch("/auth/signout" , {
                method : "POST",
                credentials : "include",
            });
            switch (res.status) {
                case 200:
                    setUser(null);
                    navigate("/login")
                    break;
                default:
                    break
            }
        }catch(e) {
             console.log("Failed to log out");
        }
    }
    return <div className="w-screen h-[8%] sticky top-0 left-0 flex justify-between items-center p-4 border-b-white border-b-2 border-solid">
        <nav>
            <Link to={"/index.html"}> Home </Link>
        </nav>
        {
            user == null ? <div>
                <Link to="/login"> Login </Link>
                <Link to="/register"> Register </Link>
            </div> : <div className="h-full flex items-center justify-center gap-2">
                <p> Hello <Link to="/me">{user.name}</Link> </p>
                <button onClick={handleLogOut}> Log Out </button>
            </div>
        }
    </div>
}