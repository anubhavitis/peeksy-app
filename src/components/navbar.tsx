import { invoke } from "@tauri-apps/api/core";
import peeksy_eyes from "../assets/peeksy_eyes.png";

const Navbar = () => {
  function closeWindow() {
    invoke("close_window");
  }

  return (
    <div className="flex justify-between">
      <div className="flex gap-2">
        <button
          className="bg-[#FF6056] w-3 h-3 rounded-full hover:bg-red-600 cursor-pointer"
          id="close-btn"
          onClick={closeWindow}
        />
        <button
          className="bg-[#FEBC2E] w-3 h-3 rounded-full hover:bg-yellow-600 cursor-pointer"
          id="close-btn"
          onClick={closeWindow}
        />
      </div>
      <div className="flex items-center gap-2 justify-center">
        <img src={peeksy_eyes} alt="Peeksy" className="w-12 h-6" />
      </div>
    </div>
  );
};

export default Navbar;
