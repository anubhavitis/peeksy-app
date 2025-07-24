import Navbar from "./navbar";
import Configs from "./configs";
import Header from "./header";

function ConfigsPage() {
  return (
    <div className="min-h-screen shadow-lg overflow-hidden text-gray-900 dark:text-gray-100 transition-all duration-200">
      <div className="flex flex-col gap-3 p-4">
        <Navbar />
        <Header />
        <Configs />
      </div>
    </div>
  );
}

export default ConfigsPage;
