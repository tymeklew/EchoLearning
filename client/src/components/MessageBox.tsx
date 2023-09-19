import {Simulate} from "react-dom/test-utils";

interface Message {
    message : string,
    error : boolean,
}
export default function MessageBox(props : Message) {
    return <div className={`flex items-center justify-center text-[20px] ${props.error ? "bg-red-500" : "bg-green-500"}`}>
        {props.message}
    </div>
}