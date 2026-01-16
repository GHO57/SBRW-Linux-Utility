import { HashRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./components/Home/Home";
import Wizard from "./components/Wizard/Wizard";
import Settings from "./components/Settings/Settings";

function App() {
    return (
        <main>
            <div className="bg-light dark:bg-dark w-full h-screen">
                <Router>
                    <div className="w-full h-full">
                        <Routes>
                            <Route path="/" element={<Home />} />
                            <Route path="/setup-wizard" element={<Wizard />} />
                            <Route path="/settings" element={<Settings />} />
                        </Routes>
                    </div>
                </Router>
            </div>
        </main>
    );
}

export default App;
