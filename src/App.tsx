import React from "react";
import "./App.css";
import { Routes, Route } from 'react-router-dom'

import Home from './views/home/Home'
import Welcome from "./views/welcome/Welcome";

const App: React.FC = () => {

 

  return (
   <>
    <Routes>
      <Route path =  "/" element = { <Welcome />} />
      <Route path="/home" element = { <Home /> } />
    </Routes>
   </>
  );
}

export default App;
