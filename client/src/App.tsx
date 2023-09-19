import {
  BrowserRouter,
  Routes,
  Route
} from "react-router-dom"
import { Home, Login, NotFound, Register, Reset, ResetPrompt } from './pages/index'
import Me from "./pages/Me"
import {NavBar} from "./components";

function App() {

  return (
    <BrowserRouter>
      <NavBar/>
      <Routes>
        <Route path="/index.html" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/me" element={<Me />} />
        <Route path="/reset" element={<ResetPrompt/>}/>
        <Route path="/reset/:reset_id" element={<Reset />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
