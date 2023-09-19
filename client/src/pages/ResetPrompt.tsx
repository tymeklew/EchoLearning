import {useState , FormEvent} from "react";
export default function ResetPrompt() {
  const [email , setEmail] = useState("");
  async function handleSubmit(e : FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const res = await fetch(`/reset?email=${email}`);
    console.log(res.status);
  }
  return <div className="w-screen h-screen flex items-center justify-center">
        <form onSubmit={handleSubmit} className="w-[20%] h-[30%] bg-gradient-to-tr rounded-md text-center gap-2">
            <h1> If a user with this email exists <br/> an email with a reset url will be send to this email</h1>
            <input type="email" className="form-element" placeholder="Email" value={email} onChange={e => setEmail(e.target.value)}/>
            <button type="submit"> Send </button>
        </form>
     </div>
}