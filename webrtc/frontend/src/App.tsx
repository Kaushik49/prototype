import './App.css'
import { Route, BrowserRouter, Routes } from 'react-router-dom';
import { Sender } from './components/sender'
import { Receiver } from './components/receiver'

// the sender route sends the user to the sender component
// the receiver router sends the user to the reciever component
function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/sender" element={<Sender />} />
        <Route path="/receiver" element={<Receiver />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
