import { useAuth } from "../contexts/AuthContext";
import Auth from "./auth";
import UserProfile from "./UserProfile";
import Navbar from "./navbar";

const AuthPage = () => {
  const { user, loading } = useAuth();

  if (loading) {
    return (
      <div className="text-gray-900 dark:text-gray-100 transition-all duration-200">
        <div className="flex flex-col gap-3 p-4">
          <Navbar />
          <div className="flex items-center justify-center min-h-[400px]">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="overflow-y-auto text-gray-900 dark:text-gray-100 transition-all duration-200">
      <div className="flex flex-col gap-3 p-4">
        <Navbar />
        <div className="flex items-center justify-center">
          {user ? <UserProfile /> : <Auth />}
        </div>
      </div>
    </div>
  );
};

export default AuthPage;
