import Navbar from "./navbar";

function HomePage() {
  return (
    <div className="min-h-screen shadow-lg overflow-hidden text-gray-900 dark:text-gray-100 transition-all duration-200">
      <div className="flex flex-col gap-3 p-4">
        <Navbar />
        <div className="text-center">
          <h1 className="text-6xl font-bold text-gray-900 dark:text-gray-100 mb-4">
            Hello World
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-400">
            Welcome to Peeksy App
          </p>
        </div>
      </div>
    </div>
  );
}

export default HomePage;
