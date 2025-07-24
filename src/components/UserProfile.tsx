import { useAuth } from "../contexts/AuthContext";

const UserProfile = () => {
  const { user, signOut, loading } = useAuth();

  if (loading) {
    return (
      <div className="flex items-center justify-center p-4">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (!user) {
    return null;
  }

  const handleSignOut = async () => {
    try {
      await signOut();
    } catch (error) {
      console.error("Error signing out:", error);
    }
  };

  return (
    <div className="w-full mx-auto px-10 flex flex-col gap-4">
      <div className="text-center">
        <div className="w-20 h-20 bg-indigo-100 dark:bg-indigo-900 rounded-full flex items-center justify-center mx-auto mb-4">
          <span className="text-2xl font-bold text-indigo-600 dark:text-indigo-400">
            {user.email?.charAt(0).toUpperCase()}
          </span>
        </div>
        <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
          Welcome back!
        </h2>
        <p className="text-gray-600 dark:text-gray-400 mt-1">{user.email}</p>
      </div>

      <button
        onClick={handleSignOut}
        className="w-full mt-6 py-2 px-4 bg-red-600 hover:bg-red-700 text-white 
                 font-medium rounded-md transition-colors duration-200
                 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
      >
        Sign Out
      </button>
    </div>
  );
};

export default UserProfile;
